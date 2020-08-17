use std::rc::Rc;

#[derive(Debug)]
pub struct RBTree<T> {
    root: Link<T>,
}

type Link<T> = Option<Rc<RBNode<T>>>;

#[derive(Debug)]
struct RBNode<T> {
    colour: Colour, 
    element: T, 
    left: Link<T>, 
    right: Link<T>,
} 

#[derive(Debug, Copy, Clone, PartialEq)]
enum Colour {
    Red,
    Black
}

impl<T> Clone for RBTree<T> {
    fn clone(&self) -> Self {
        RBTree { root: self.root.clone() }
    }
}

impl<T: Clone> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub fn leaf(element: T) -> Self {
        RBTree {
            root: Some( Rc::new(
                RBNode {
                    colour: Colour::Red,
                    element,
                    left: None,
                    right: None,
                }
            ))
        }
    }

    fn tree(colour: Colour, value: T, left: &Self, right: &Self) -> Self {
        RBTree {
            root: Some( Rc::new(
                RBNode {
                    colour, 
                    element: value.clone(), 
                    left: left.root.clone(), 
                    right: right.root.clone(),
                }
            )),
        }
    }
    
    fn from_node(node: &Rc<RBNode<T>>) -> Self {
        RBTree {
            root: Some( node.clone() ),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn root(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.element)
    }

    fn root_colour(&self) -> Colour {
        self.root.as_ref().map_or(
            Colour::Black,
            |node| node.colour
        )
    }
    pub fn left(&self) -> Self {
        assert!(!self.is_empty());
        Self { root: self.root.as_ref().and_then(|node| node.left.clone()) }
    }

    pub fn right(&self) -> Self {
        assert!(!self.is_empty());
        Self { root: self.root.as_ref().and_then(|node| node.right.clone()) } 
    }

    pub fn inserted(&self, x: T) -> Self 
    where
        T: PartialOrd,
    {
        RBTree { root: link_inserted(&self.root, x) }
//        match &self.root {
//            None => RBTree::leaf(x),
//            Some(root) => {
//                let t = root.ins(x);
//                RBTree {
//                    root:  Some(Rc::new(
//                        RBNode {
//                            colour: Colour::Black,
//                            element: t.element.clone(),
//                            left: t.left.clone(),
//                            right: t.right.clone()
//                        }
//                    ))
//                }
//            }
//        }
        // let t = self.ins(x);
        // RBTree::tree(Colour::Black, t.root().unwrap().clone(), &t.left(), &t.right())
    }

    pub fn inserted_or_replaced(&self, x: T) -> Self 
    where
        T: PartialOrd,
    {
        RBTree { 
            root: link_inserted_or_replaced(&self.root, x) 
        }
        // let t = self.ins_or_rep(x);
        // RBTree::tree(Colour::Black, t.root().unwrap().clone(), &t.left(), &t.right())
    }

    pub fn contains<U>(&self, x: &U) -> bool
    where
        T: PartialOrd<U>,
    {
        self.root.as_ref().map_or_else(|| false, |node| node.contains(x))
    }

    pub fn get<U>(&self, x: &U) -> Option<&T>
    where
        T: PartialOrd<U>
    {
        self.root.as_ref().and_then(|node| node.get(x))
    }

    pub fn get_or_default<'a, U>(&'a self, x: &U, default: &'a T) -> &'a T
    where
        T: PartialOrd<U>
    {
        match self.get(x) {
            Some(v) => v,
            None => default,
        }
    }

    fn ins(&self, x: T) -> Self
    where
        T: PartialOrd,
    {
        match &self.root {
            None => RBTree::leaf(x),
            Some(node) => {
                if x < node.element {
                    balance(
                        node.colour, 
                        node.element.clone(), 
                        &RBTree { root: node.left.clone() }.ins(x),
                        &RBTree { root: node.right.clone() }
                    )
                } else if x > node.element {
                    balance(
                        node.colour, 
                        node.element.clone(), 
                        &RBTree { root: node.left.clone() },
                        &RBTree { root: node.right.clone() }.ins(x)
                    )
                } else {
                    RBTree { root: self.root.clone() }
                }
            }
        }
    }

    fn ins_or_rep(&self, x: T) -> Self
    where
        T: PartialOrd,
    {
        match &self.root {
            None => RBTree::leaf(x),
            Some(node) => {
                if x < node.element {
                    balance(
                        node.colour, 
                        node.element.clone(), 
                        &RBTree { root: node.left.clone() }.ins_or_rep(x),
                        &RBTree { root: node.right.clone() }
                    )
                } else if x > node.element {
                    balance(
                        node.colour, 
                        node.element.clone(), 
                        &RBTree { root: node.left.clone() },
                        &RBTree { root: node.right.clone() }.ins_or_rep(x)
                    )
                } else {
                    RBTree { 
                        root: Some(Rc::new(
                            RBNode {
                                colour: node.colour,
                                element: x,
                                left: node.left.clone(),
                                right: node.right.clone(),
                                      }
                                          ))
                    }
                }
            }
        }
    }

    fn doubled_left(&self) -> bool {
        !self.is_empty() 
        && self.root_colour() == Colour::Red
        && !self.left().is_empty()
        && self.left().root_colour() == Colour::Red
    }

    fn doubled_right(&self) -> bool {
        !self.is_empty() 
        && self.root_colour() == Colour::Red
        && !self.right().is_empty()
        && self.right().root_colour() == Colour::Red
    }

    fn paint(&self, c: Colour) -> Self {
        assert!(!self.is_empty());
        RBTree::tree(
            c,
            self.root().unwrap().clone(),
            &self.left(),
            &self.right()
        )
    }
}

fn balance<T>(c: Colour, x: T, left: &RBTree<T>, right: &RBTree<T>) -> RBTree<T>
where
    T: Clone,
{
    use Colour::*;
    if c == Black && left.doubled_left() {
        RBTree::tree(
            Red,
            left.root().unwrap().clone(),
            &left.left().paint(Black),
            &RBTree::tree(Black, x, &left.right(), right)
        )
    } else if c == Black && left.doubled_right() {
        RBTree::tree(
            Red,
            left.right().root().unwrap().clone(),
            &RBTree::tree(
                Black,
                left.root().unwrap().clone(),
                &left.left(),
                &left.right().left()
            ),
            &RBTree::tree(
                Black,
                x,
                &left.right().right(),
                right
            )
        )
    } else if c == Black && right.doubled_left() {
        RBTree::tree(
            Red,
            right.left().root().unwrap().clone(),
            &RBTree::tree(
                Black,
                x,
                left,
                &right.left().left()
            ),
            &RBTree::tree(
                Black,
                right.root().unwrap().clone(),
                &right.left().right(),
                &right.right()
            )
        )
    } else if c == Black && right.doubled_right() {
        RBTree::tree(
            Red,
            right.root().unwrap().clone(),
            &RBTree::tree(
                Black,
                x,
                left,
                &right.left()
            ),
            &right.right().paint(Black)
        )
    } else {
        RBTree::tree(c, x, left, right)        
    }
}

impl<T> RBNode<T> {

    fn left(&self) -> Option<&Rc<Self>> {
       self.left.as_ref().map(|node| node) 
    }

    fn right(&self) -> Option<&Rc<Self>> {
        self.right.as_ref()
    }

    fn contains<U>(&self, x: &U) -> bool
    where
        T: PartialOrd<U>,
    {
        if &self.element > x {
            self.left.as_ref().map_or(false, |node| node.contains(x))
        } else if &self.element < x {
            self.right.as_ref().map_or(false, |node| node.contains(x))
        } else {
            true
        }
    }

    fn get<U>(&self, x: &U) -> Option<&T> 
    where
        T: PartialOrd<U>,
    {
        if &self.element > x {
            self.left.as_ref().and_then(|node| node.get(x))
        } else if &self.element < x {
            self.right.as_ref().and_then(|node| node.get(x))
        } else {
            Some(&self.element)
        }
    }
    fn doubled_left(&self) -> bool {
        self.colour == Colour::Red
        && self.left.as_ref().map_or(
            false, |node| node.colour == Colour::Red)
    }

    fn doubled_right(&self) -> bool {
        self.colour == Colour::Red
        && self.right.as_ref().map_or(
            false, |node| node.colour == Colour::Red)
    }

}
fn link_inserted<T>(link: &Link<T>, x: T) -> Link<T>
where
    T: Clone + PartialOrd,
{
    let new_link = sorted_insert(link, x);
    paint_link(&new_link, Colour::Black)
}

fn link_inserted_or_replaced<T>(link: &Link<T>, x: T) -> Link<T>
where
    T: Clone + PartialOrd,
{
    let new_link = sorted_insert_or_replace(link, x);
    paint_link(&new_link, Colour::Black)
}

fn sorted_insert<T>(link: &Link<T>, x: T) -> Link<T>
where
    T: Clone + PartialOrd,
{
    match link {
        None => make_leaf_link(x),
        Some(node) => {
            if x < node.element {
                balance_link(
                    node.colour,
                    node.element.clone(),
                    sorted_insert(&node.left.clone(), x),
                    node.right.clone()
                )
            } else if x > node.element {
                balance_link(
                    node.colour,
                    node.element.clone(),
                    node.left.clone(),
                    sorted_insert(&node.right.clone(), x)
                )
            } else {
                link.clone()
            }
        }
    }
}

fn sorted_insert_or_replace<T>(link: &Link<T>, x: T) -> Link<T>
where
    T: Clone + PartialOrd,
{
    match link {
        None => make_leaf_link(x),
        Some(node) => {
            if x < node.element {
                balance_link(
                    node.colour,
                    node.element.clone(),
                    sorted_insert_or_replace(&node.left.clone(), x),
                    node.right.clone()
                )
            } else if x > node.element {
                balance_link(
                    node.colour,
                    node.element.clone(),
                    node.left.clone(),
                    sorted_insert_or_replace(&node.right.clone(), x)
                )
            } else {
                make_link(
                    node.colour,
                    x,
                    node.left.clone(),
                    node.right.clone()
                )
            }
        }
    }
}

//fn make_empty_node<T>() -> Rc<RBNode<T>> {
//    Rc::new(RBNode::Empty)
//}
//
fn make_leaf_link<T>(element: T) -> Link<T> {
    Some(Rc::new(
        RBNode {
            colour: Colour::Red, 
            element, 
            left: None,
            right: None 
        }
    ))
}

fn make_link<T>(colour: Colour, element: T, left: Link<T>, right: Link<T>) -> Link<T> {
    Some(Rc::new(
        RBNode { colour, element, left, right }
    ))
}

fn balance_link<T>(c: Colour, x: T, left: Link<T>, right: Link<T>) -> Link<T>
where
    T: Clone,
{
    use Colour::*;
    if c == Black && doubled_left(&left) {
        let node = left.unwrap();
        make_link(
            Red,
            node.element.clone(),
            paint_link(&node.left, Black),
            make_link(Black, x, node.right.clone(), right)
        )
    } else if c == Black && doubled_right(&left) {
        let node = left.unwrap();
        let node_right = node.right.as_ref().unwrap();
        make_link(
            Red,
            node_right.element.clone(),
            make_link(
                Black,
                node.element.clone(),
                node.left.clone(),
                node_right.left.clone()
            ),
            make_link(
                Black,
                x,
                node_right.right.clone(),
                right
            )
        )
    } else if c == Black && doubled_left(&right) {
        let node = right.unwrap();
        let node_left = node.left.as_ref().unwrap();
        make_link(
            Red,
            node_left.element.clone(),
            make_link(
                Black,
                x,
                left,
                node_left.left.clone()
            ),
            make_link(
                Black,
                node.element.clone(),
                node_left.right.clone(),
                node.right.clone()
            )
        )
    } else if c == Black && doubled_right(&right) {
        let node = right.unwrap();
        make_link(
            Red,
            node.element.clone(),
            make_link(
                Black,
                x,
                left,
                node.left.clone()
            ),
            paint_link(&node.right, Black)
        )
    } else {
        make_link(c, x, left, right)
    }
}

fn doubled_left<T>(link: &Link<T>) -> bool {
    match link {
        None => false,
        Some(node) => {
            node.colour == Colour::Red
            && node.left.as_ref().map_or(false, |node| node.colour == Colour::Red)
        }
    }
}

fn doubled_right<T>(link: &Link<T>) -> bool {
    match link {
        None => false,
        Some(node) => {
            node.colour == Colour::Red
            && node.right.as_ref().map_or(false, |node| node.colour == Colour::Red)
        }
    }
}

fn paint_link<T>(link: &Link<T>, colour: Colour) -> Link<T>
where
    T: Clone,
{
    link.as_ref().map(|node| Rc::new(
            RBNode { 
                colour,
                element: node.element.clone(),
                left: node.left.clone(),
                right: node.right.clone()
            }
        )
    )
}

impl<T: PartialEq + Clone> PartialEq for RBTree<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_empty() && other.is_empty() {
            return true;
        } else if self.is_empty() || other.is_empty() {
            return false;
        }
        
        if self.root() != other.root() {
            return false;
        }
        if self.left() != other.left() {
            return false;
        } 
        if self.right() != other.right() {
            return false;
        }

        true
    }
}

impl<T: PartialEq> PartialEq for RBNode<T> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_tree() {
        let tree = RBTree::<i32>::new();

        assert!(tree.is_empty());
        assert_eq!(tree.root(), None);
    }

    #[test]
    fn tree_creates_nonempty_tree() {
        let empty_tree = RBTree::new();
        let tree = RBTree::tree(Colour::Red, 5, &empty_tree, &empty_tree);

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&5));
    }

    #[test]
    fn left_returns_left_tree() {
        let tree = RBTree::tree(
            Colour::Black,
            "root",
            &RBTree::leaf("left"),
            &RBTree::leaf("right")
        );

        println!("{:?}", tree);
        assert_eq!(tree.left(), RBTree::leaf("left"));
        assert_eq!(tree.right(), RBTree::leaf("right"));
    }

    #[test]
    fn inserted_on_empty_tree_returns_larger_tree() {
        let empty_tree = RBTree::new();

        assert_eq!(empty_tree.root(), None);
        assert_eq!(empty_tree.root_colour(), Colour::Black);
        let t1 = empty_tree.inserted("b");

        assert_eq!(t1.root(), Some(&"b"));
        assert_eq!(t1.root_colour(), Colour::Black);

    }

    #[test]
    fn inserted_two_times() {
        let empty_tree = RBTree::new();

        let t1 = empty_tree.inserted("b");


        let t2 = t1.inserted("a");
        assert_eq!(t2.root(), Some(&"b"));
        assert_eq!(t2.root_colour(), Colour::Black);
        assert_eq!(t2.left().root(), Some(&"a"));
        assert_eq!(t2.left().root_colour(), Colour::Red);
        assert!(t2.right().is_empty());
    }

    #[test]
    fn inserted_three_times() {
        let empty_tree = RBTree::new();
        assert!(!empty_tree.contains(&"a"));

        let t1 = empty_tree.inserted("c");
        assert!(!t1.contains(&"a"));
        let t2 = t1.inserted("a");
        assert!(t2.contains(&"a"));
        let t = t2.inserted("b");
        //  [(B, b) 
        //      [(R, a) 
        //          [] 
        //          []
        //      ]
        //      [(R, c) 
        //          [] 
        //          []
        //      ]
        //  ]
        assert_eq!(t.root(), Some(&"b"));
        assert_eq!(t.root_colour(), Colour::Black);
        assert_eq!(t.left().root(), Some(&"a"));
        assert_eq!(t.left().root_colour(), Colour::Black);
        assert_eq!(t.right().root(), Some(&"c"));
        assert_eq!(t.right().root_colour(), Colour::Black);
    }

    #[derive(Clone, Debug, PartialEq)]
    struct KV<K, V>(K, V);

    impl<K, V> PartialOrd for KV<K, V>
    where
        K: PartialOrd,
        V: PartialEq,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    #[test]
    fn inserted_or_replaced() {
        let empty_tree = RBTree::new();

        let t1 = empty_tree.inserted_or_replaced(KV(4, "a"));
        let t2 = t1.inserted_or_replaced(KV(3, "b"));
        let t3 = t2.inserted_or_replaced(KV(5, "c"));
        let t4 = t3.inserted_or_replaced(KV(4, "d"));

        assert_eq!(t3.root(), Some(&KV(4, "a")));
        assert_eq!(t4.root(), Some(&KV(4, "d")));
    }

    #[test]
    fn get_or_default() {
        let t1 = RBTree::new();

        assert_eq!(t1.get_or_default(&5, &7), &7);
    }
}
