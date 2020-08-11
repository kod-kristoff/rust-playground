use std::collections::{HashMap, HashSet};
use async_stream::stream;
use futures::stream::Stream;
use futures::pin_mut;
use futures::stream::StreamExt;

use ds_13::rb_map::RBMap;
use ds_13::list::{List, ListIterator};
use crate::domain::models::{Rule, Edge, Chart};
use crate::utilities::container::leftcorners_dict;

pub fn parse<'a>(grammar: &'a [Rule], cat: &'a str, sentence: &[&'a str]) -> impl Iterator<Item = Tree> + 'a {
    let chart = earley(grammar, sentence);
    extract_tree(chart, cat)
}

pub fn earley<'a>(grammar: &'a [Rule], input: &[&'a str]) -> Chart<'a> {
    let leftcorners = leftcorners_dict(grammar);

    let mut chart: Vec<HashMap<Option<&str>, HashSet<Edge>>> = Vec::new();
    {
        let mut entry_0 = HashMap::new();
        entry_0.insert(None, HashSet::new());
        chart.push(entry_0);
    }

    for (k, sym) in input.iter().enumerate() {
        let k = k + 1;

        let mut lc_edgesets = HashMap::new();

        // Scan
        let mut agenda = vec!(Edge {
            start: k-1,
            end: k,
            lhs: sym,
            rhs: Vec::new(),
            dot: 0,
        });

        while agenda.len() > 0 {
            // println!("agenda = {:?}", agenda);

            let edge = match agenda.pop() {
                Some(edge) => edge,
                None => panic!("no edge")
            };

            let leftc = match edge.is_passive() {
                true => None,
                false => Some(edge.rhs[edge.dot])
            };
            let edgeset = lc_edgesets.entry(leftc).or_insert(HashSet::<Edge>::new());

            if !edgeset.contains(&edge) {
                if edge.is_passive() {
                    // Predict
                    if leftcorners.contains_key(edge.lhs) {
                        let rules = &leftcorners[edge.lhs];
                        for rule in rules {
                            agenda.push(
                                Edge {
                                    start: edge.start,
                                    end: k,
                                    lhs: &rule.lhs,
                                    rhs: rule.rhs.iter().map(String::as_str).collect(),
                                    dot: 1,
                                }
                            );
                        }
                    }

                    // Complete
                    if chart[edge.start].contains_key(&Some(edge.lhs)) {
                        for e in &chart[edge.start][&Some(edge.lhs)] {
                            agenda.push(
                                Edge {
                                    start: e.start,
                                    end: k,
                                    lhs: e.lhs,
                                    rhs: e.rhs.iter().map(|x| *x).collect(),
                                    dot: e.dot + 1,
                                }
                            );
                        }
                    }
                } // if edge is passive
                edgeset.insert(edge);
            } // if edge not in edgeset
        } // while agenda
        chart.push(lc_edgesets);
    } // for input

    let mut result = Chart::new();
    for lc_edgeset in chart {
        let mut part = Vec::new();
        for edge in lc_edgeset.get(&None).unwrap() {
            part.push(edge.clone())
        }
        result.chart.push(part);
    }
    result
}

pub fn extract_tree<'a>(chart: Chart<'a>, cat: &'a str) -> impl Iterator<Item = Tree> + 'a {
    let start: usize = 0;
    let end = chart.chart.len() - 1;
    let topdowns = {
        let mut topdowns = RBMap::new();
        let empty_list = List::new();
        for edgeset in chart.chart.iter() {
            for edge in edgeset {
                if edge.is_passive() {
                    let edges = topdowns.get_or_default(&(edge.lhs, edge.start), &empty_list).pushed_front(edge.clone());
                    topdowns = topdowns.inserted_or_replaced((edge.lhs, edge.start), edges);
                }
            }
        }
        topdowns
    };
    let mut result = List::new(); 
    for (tree, _) in yield_tree(topdowns.clone(), cat, start, Box::new(move |e| e == end)) {
        result = result.pushed_front(tree);
    }
    result.into_iter()
}

fn yield_tree<'a>(topdowns: TopdownMap<'a>, lhs: &'a str, start: usize, test_end: Box<dyn Fn(usize) -> bool>) -> List<(Tree, usize)> {
    let mut result = List::new();
    for edge in topdowns.get_or_default(&(lhs, start), &List::new()) {
        if test_end(edge.end) {
            let yield_children = yield_children(
                topdowns.clone(),
                edge.rhs.clone(),
                0,
                start,
                edge.end
            );
            for children in yield_children {
                result = result.pushed_front((Tree::new(lhs, children), edge.end));
            }
        }
    }
    result
}

fn yield_children<'a>(topdowns: TopdownMap<'a>, rhs: Vec<&'a str>, dot: usize, start: usize, end: usize) -> List<List<Tree>> {
    let mut result = List::new();
    if rhs.is_empty() {
        result = result.pushed_front(List::new());
    } else if start == end && dot == rhs.len() {
        result = result.pushed_front(List::new());
    } else if start < end && dot < rhs.len() {
        let yield_tree = if dot == rhs.len() - 1 {
            yield_tree(
                topdowns.clone(),
                rhs[dot],
                start,
                Box::new(move |e| e == end)
            )
        } else {
            yield_tree(
                topdowns.clone(),
                rhs[dot],
                start,
                Box::new(move |e| e < end)
            )
        };
        for (tree, mid) in yield_tree {
            let yield_children = yield_children(
                topdowns.clone(),
                rhs.clone(),
                dot + 1,
                mid,
                end
            );
            for trees in yield_children {
                result = result.pushed_front(List::cons(tree.clone(), &trees));
            }
        }
    }
    result
}

type TopdownMap<'a> = RBMap<(&'a str, usize), List<Edge<'a>>>;

pub struct ParseTreeIterator<'a> {
    chart: Chart<'a>,
    cat: &'a str,
    topdowns: TopdownMap<'a>,
    yield_tree: YieldTreeIterator<'a>,
}

impl<'a> ParseTreeIterator<'a> {
    fn new(chart: Chart<'a>, cat: &'a str) -> Self {
        let topdowns = {
            let mut topdowns = RBMap::new();
            let empty_list = List::new();
            for edgeset in chart.chart.iter() {
                for edge in edgeset {
                    if edge.is_passive() {
                        let edges = topdowns.get_or_default(&(edge.lhs, edge.start), &empty_list).pushed_front(edge.clone());
                        topdowns = topdowns.inserted_or_replaced((edge.lhs, edge.start), edges);
                    }
                }
            }
            topdowns
        };
        let end = chart.chart.len() - 1;
        let yield_tree = YieldTreeIterator::new(topdowns.clone(), cat, 0, Box::new(move |e| e == end) );
        ParseTreeIterator { chart, cat, topdowns, yield_tree }
    }
}

impl<'a> Iterator for ParseTreeIterator<'a> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        println!("ParseTreeIterator::next");
        match self.yield_tree.next() {
            None => None,
            Some(tree) => Some(tree),
        }
    }
}

struct YieldTreeIterator<'a> {
    topdowns: TopdownMap<'a>,
    lhs: &'a str,
    start: usize,
    test_end: Box<dyn Fn(usize) -> bool>,
    edge_iter: ListIterator<Edge<'a>>,
    yield_children: YieldChildrenIterator<'a>,
    edge: Option<Edge<'a>>,
}

impl<'a> YieldTreeIterator<'a> {
    fn new(topdowns: TopdownMap<'a>, lhs: &'a str, start: usize, test_end: Box<dyn Fn(usize) -> bool>) -> Self {
        let yield_children = YieldChildrenIterator::empty();
        let edge_iter = topdowns.get_or_default(&(lhs, start), &List::new()).into_iter();
        let edge = None;
        YieldTreeIterator { topdowns, lhs, start, test_end, edge_iter, yield_children, edge }

    }
}

impl<'a> Iterator for YieldTreeIterator<'a> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        println!("YieldTreeIterator::next");
        if self.yield_children.is_empty() {
            loop {
                match &self.edge_iter.next() {
                    None => { return None; },
                    Some(edge) => {
                        println!("{:?}", edge);
                        if (self.test_end)(edge.end) {
                            self.edge = Some(edge.clone());
                            self.yield_children = YieldChildrenIterator::new(
                                self.topdowns.clone(),
                                edge.rhs.clone(),
                                0,
                                self.start,
                                edge.end
                            );
                            break;
                        }
                    }
                }
            }
        }
        if !self.yield_children.is_empty() {
            match self.yield_children.next() {
                None => (),
                Some(children) => {
                    return Some(Tree::new(self.lhs, children));
                }
            } 
        }
        None
    }
}

struct YieldChildrenIterator<'a> {
    rhs: Option<Vec<&'a str>>,
    //yield_tree: YieldTreeIterator<'a>,
    mid: usize,
    tree: Option<Tree>,
    
}

impl<'a> YieldChildrenIterator<'a> {
    fn empty() -> Self {
        YieldChildrenIterator { 
            rhs: None,
            //yield_tree: YieldTreeIterator<'a>,
            mid: 0,
            tree: None,
        }
    }

    fn new(topdowns: TopdownMap<'a>, rhs: Vec<&'a str>, dot: usize, start: usize, end: usize) -> Self {
        let yield_tree = if dot == rhs.len() - 1 {
            YieldTreeIterator::new(
                topdowns.clone(),
                rhs[dot],
                start,
                Box::new( move |e| e == end )
            )
        } else {
            YieldTreeIterator::new(
                topdowns.clone(),
                rhs[dot],
                start,
                Box::new( move |e| e < end )
            )
        };
        YieldChildrenIterator {
            rhs: Some(rhs),
            //yield_tree: yield_tree,
            mid: 0,
            tree: None,
        }
    }

    fn is_empty(&self) -> bool {
        match &self.rhs {
            None => true,
            _ => false,
        }
    }
}

impl<'a> Iterator for YieldChildrenIterator<'a> {
    type Item = List<Tree>;

    fn next(&mut self) -> Option<Self::Item> {
        println!("YieldChildrenIterator::next");
//        match self.yield_tree.next() {
//            None => None,
//            Some(tree) => List::cons(tree, List::new()),
//        }
        None
    }
}
#[derive(Clone, Debug)]
pub struct Tree {
    root: String,
    children: List<Tree>,
}

impl Tree {
    pub fn new(root: &str, children: List<Tree>) -> Self {
        Tree { 
            root: root.to_string(),  
            children: children 
        }
    }

    pub fn leaf(root: &str) -> Self {
        Tree {
            root: root.to_string(),
            children: List::new()
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(fmt, "{}", self.root)
        } else {
            write!(fmt, "{}[", self.root)?;
            let mut children = (&self.children).into_iter();
            let mut prev = children.next().unwrap();
            while let Some(next) = children.next() {
                write!(fmt, "{} ", prev)?;
                prev = next;
            }
            //for child in &self.children {
            //    write!(fmt, "{} ", child)?;
            //}
            write!(fmt, "{}]", prev)
        }
    }
}
