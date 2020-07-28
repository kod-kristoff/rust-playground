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

#[derive(Debug)]
enum Colour {
    Red,
    Black
}

impl<T> RBTree<T> {
    pub fn new() -> Self {
        RBTree { root: Rc::new( RBNode::<T>::Empty ) }
    }

    pub fn leaf(value: T) -> Self {
        RBTree {
            root: Rc::new(
                RBNode::Node(
                    value,
                    Self::new().root,
                    Self::new().root
                )
            )
        }
    }

    pub fn tree(value: T, left: &Self, right: &Self) -> Self {
        RBTree {
            root: Rc::new(
                RBNode::Node(value, left.root.clone(), right.root.clone())
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
            RBNode::Empty => None,
            RBNode::Node(_c, v, _left, _right) => Some(v),
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

    pub fn inserted(&self, v: T) -> Self {
        Self::leaf(v)
    }
}

impl<T> RBNode<T> {
    fn left(&self) -> &Rc<Self> {
        match self {
            Self::Node(_v, left, _right) => left,
            _ => panic!("Can't take left from empty node.")
        }
    }

    fn right(&self) -> &Rc<Self> {
        match self {
            Self::Node(_v, _left, right) => right,
            _ => panic!("Can't take right from empty node.")
        }
    }
}

impl<T: PartialEq> PartialEq for RBTree<T> {
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
        let tree = RBTree::tree(5, &empty_tree, &empty_tree);

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&5));
    }

    #[test]
    fn left_returns_left_tree() {
        let tree = RBTree::tree(
            "root",
            &RBTree::leaf("left"),
            &RBTree::leaf("right")
        );

        println!("{:?}", tree);
        assert_eq!(tree.left(), RBTree::leaf("left"));
        assert_eq!(tree.right(), RBTree::leaf("right"));
    }

    #[test]
    fn inserted_returns_larger_tree() {
        let empty_tree = RBTree::new();

        let t1 = empty_tree.inserted("b");

        assert_eq!(t1.root(), Some(&"b"));

        let t2 = t1.inserted("a")
    }
}
