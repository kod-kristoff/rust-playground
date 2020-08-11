use std::rc::Rc;
use crate::list;
use crate::list::List;

#[derive(Debug)]
pub struct Tree<T> {
    root: Rc<TreeNode<T>>
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        let root = Rc::new(TreeNode::Empty);
        Tree { root }
    }

    pub fn leaf(x: T) -> Self {
        Tree {
            root: Rc::new(
                TreeNode::Node(x, List::new())
            )
        }
    }

    pub fn tree(x: T, children: &List<Self>) -> Self {
        Tree {
            root: Rc::new(
                TreeNode::Node(x, children.clone())
            )
        }
    }

    pub fn is_empty(&self) -> bool {
        match &*self.root {
            TreeNode::Empty => true,
            _ => false,
        }
    }

    pub fn root(&self) -> Option<&T> {
        match &*self.root {
            TreeNode::Empty => None,
            TreeNode::Node(x, _children) => Some(x),
        }
    }

    pub fn children(&self) -> &List<Tree<T>> {
        match &*self.root {
            TreeNode::Empty => panic!("Empty tree"),
            TreeNode::Node(_x, children) => children,
        }
    }
}

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Tree { root: Rc::clone(&self.root) }
    }
}

impl<T> PartialEq for Tree<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match &*self.root {
            TreeNode::Empty => other.is_empty(),
            TreeNode::Node(x, xs) => {
                match &*other.root {
                    TreeNode::Empty => false,
                    TreeNode::Node(y, ys) => {
                        x == y && xs == ys
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum TreeNode<T> {
    Empty,
    Node(T, List<Tree<T>>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_tree() {
        let tree = Tree::<&str>::new();

        assert!(tree.is_empty());
        assert_eq!(tree.root(), None);
    }

    #[test]
    fn leaf_creates_tree_w_no_children() {
        let tree = Tree::leaf(5);

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&5));
        assert!(tree.children().is_empty());
    }

    #[test]
    fn tree_creates_tree_w_no_children() {
        let tree = Tree::tree("a", &List::new());

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&"a"));
        assert!(tree.children().is_empty());
    }

    #[test]
    fn tree_creates_tree_w_children() {
        let children = list!(
            Tree::leaf("b"),
            Tree::leaf("c")
        );
        let tree = Tree::tree("a", &children);

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&"a"));
        assert!(!tree.children().is_empty());
        assert_eq!(tree.children(), &children);
    }

    #[test]
    fn test_partial_eq() {
        let t1 = Tree::<i32>::new();
        let t2 = Tree::<i32>::new();
        let t3 = Tree::leaf(4);
        let t4 = Tree::tree(
            4,
            &list!(
                Tree::leaf(5)
            )
        );
        let t5 = Tree::tree(
            4,
            &list!(
                Tree::leaf(5)
            )
        );
        let t6 = Tree::tree(
            4,
            &list!(
                Tree::leaf(6)
            )
        );
        assert!(t1 == t2);
        assert!(t1 != t3);
        assert!(t1 != t4);
        assert!(t3 != t4);
        assert!(t3 == Tree::leaf(4));
        assert!(t3 != Tree::leaf(5));
        assert!(t4 == t5);
        assert!(t4 != t6);
    }
}
