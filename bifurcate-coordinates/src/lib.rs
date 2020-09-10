pub trait BifurcateCoordinate {
    fn is_empty(&self) -> bool;
}

pub fn weight_recursive<C>(c: C) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    struct EmptyCoord;

    impl BifurcateCoordinate for EmptyCoord {
        fn is_empty(&self) -> bool {
            true
        }
    }
    #[test]
    fn empty_coord_is_empty() {
        let c = EmptyCoord {};

        assert!(c.is_empty());
    }

    #[test]
    fn weight_recursive_on_empty_coord_returns_0() {
        let c = EmptyCoord {};

        assert_eq!(weight_recursive(c), 0);
    }

    struct Node {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    impl Node {
        fn new() -> Self {
            Node {
                left: None,
                right: None,
            }
        }

        fn both(left: Self, right: Self) -> Self {
            Node {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            }
        }
    }
    impl BifurcateCoordinate for Node {
        fn is_empty(&self) -> bool {
            self.left.is_none() && self.right.is_none()
        }
    }

    #[test]
    fn empty_node_is_empty() {
        let n = Node::new();

        assert!(n.is_empty());
    }

    #[test]
    fn non_empty_node_is_not_empty() {
        let n = Node::both(
            Node::new(),
            Node::new()
        );

        assert!(!n.is_empty());
    }
}
