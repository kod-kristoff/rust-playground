use std::rc::Rc;

#[derive(Debug)]
pub struct RBTree<T> {
    root: Rc<RBNode<T>>,
}

#[derive(Debug)]
enum RBNode<T> {
    Empty,
    Node(Colour, T, Rc<RBNode<T>>, Rc<RBNode<T>>)
} 

#[derive(Debug, Copy, Clone, PartialEq)]
enum Colour {
    Red,
    Black
}

impl<T: Clone> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: Rc::new( RBNode::<T>::Empty ) }
    }

    pub fn leaf(value: T) -> Self {
        RBTree {
            root: Rc::new(
                RBNode::Node(
                    Colour::Red,
                    value,
                    Self::new().root,
                    Self::new().root
                )
            )
        }
    }

    fn tree(colour: Colour, value: T, left: &Self, right: &Self) -> Self {
        RBTree {
            root: Rc::new(
                RBNode::Node(colour, value.clone(), Rc::clone(&left.root), Rc::clone(&right.root))
            ),
        }
    }
    
    fn from_node(node: &Rc<RBNode<T>>) -> Self {
        RBTree {
            root: node.clone()
        }
    }

    pub fn is_empty(&self) -> bool {
        match &*self.root {
            RBNode::Empty => true,
            _ => false,
        }
    }

    pub fn root(&self) -> Option<&T> {
        match &*self.root {
            RBNode::Empty => None,
            RBNode::Node(_c, v, _left, _right) => Some(v),
        }
    }

    fn root_colour(&self) -> Colour {
        match &*self.root {
            RBNode::Empty => Colour::Black,
            RBNode::Node(c, _v, _left, _right) => *c,
        }
    }
    pub fn left(&self) -> Self {
        // assert!(!self.is_empty());
        Self::from_node(self.root.left()) 
    }

    pub fn right(&self) -> Self {
        assert!(!self.is_empty());
        Self::from_node(self.root.right()) 
    }

    pub fn inserted(&self, x: T) -> Self 
    where
        T: PartialOrd,
    {
        let t = self.ins(x);
        RBTree::tree(Colour::Black, t.root().unwrap().clone(), &t.left(), &t.right())
    }

    pub fn contains<U>(&self, x: &U) -> bool
    where
        T: PartialOrd<U>,
    {
        self.root.contains(x)
    }

    pub fn get<U>(&self, x: &U) -> Option<&T>
    where
        T: PartialOrd<U>
    {
        self.root.get(x)
    }

    fn ins(&self, x: T) -> Self
    where
        T: PartialOrd,
    {
        match &*self.root {
            RBNode::Empty => RBTree::leaf(x),
            RBNode::Node(c, y, left, right) => {
                if x < *y {
                    balance(
                        *c, 
                        y.clone(), 
                        &RBTree::from_node(left).ins(x),
                        &RBTree::from_node(right)
                    )
                } else if x > *y {
                    balance(
                        *c, 
                        y.clone(), 
                        &RBTree::from_node(left), 
                        &RBTree::from_node(right).ins(x)
                    )
                } else {
                    RBTree::from_node(&self.root)
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

    fn left(&self) -> &Rc<Self> {
        match self {
            Self::Node(_c, _v, left, _right) => left,
            _ => panic!("Can't take left from empty node.")
        }
    }

    fn right(&self) -> &Rc<Self> {
        match self {
            Self::Node(_c, _v, _left, right) => right,
            _ => panic!("Can't take right from empty node.")
        }
    }

    fn contains<U>(&self, x: &U) -> bool
    where
        T: PartialOrd<U>,
    {
        match self {
            Self::Empty => false,
            Self::Node(_c, y, left, right) => {
                if y > x {
                    left.contains(x)
                } else if y < x {
                    right.contains(x)
                } else {
                    true
                }
            }
        }
    }

    fn get<U>(&self, x: &U) -> Option<&T> 
    where
        T: PartialOrd<U>,
    {
        match self {
            Self::Empty => None,
            Self::Node(_c, y, left, right) => {
                if y > x {
                    left.get(x)
                } else if y < x {
                    right.get(x)
                } else {
                    Some(y)
                }
            }
        }
    }
}

fn make_empty_node<T>() -> Rc<RBNode<T>> {
    Rc::new(RBNode::Empty)
}

fn make_leaf_node<T>(x: T) -> Rc<RBNode<T>> {
    Rc::new(RBNode::Node(Colour::Red, x, make_empty_node(), make_empty_node()))
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
}
