use std::fmt;

#[derive(Debug)]
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T: Copy, U: Copy> Copy for Pair<T, U> {}

// impl<T: Copy, U: Copy> Clone for Pair<T, U> {
//     fn clone(&self) -> Self {
//         Pair { first: self.first, second: self.second }
//     }
// }

impl<T: Clone, U: Clone> Clone for Pair<T, U> {
    fn clone(&self) -> Self {
        Pair {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }
}

impl<T: fmt::Display, U: fmt::Display> fmt::Display for Pair<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

impl<T: PartialEq, U: PartialEq> PartialEq for Pair<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}
pub fn make_pair<T, U>(t: T, u: U) -> Pair<T, U> {
    Pair { first: t, second: u }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let p1 = make_pair(1, 2);
        let p2 = make_pair(1, 2);

        assert_eq!(p1, p2);

        assert!(p1 != make_pair(2, 1));
        let p3 = make_pair(3, "one");

        assert!(make_pair(5, "i") != p3);
    }
}
