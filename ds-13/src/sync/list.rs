use std::fmt::Debug;
use std::fmt;
use std::sync::Arc;
use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Link<T>,
}


impl<T> List<T> 
{
    pub fn empty() -> List<T> {
        List { head: None }
    }

    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn cons(element: T, tail: &List<T>) -> List<T> {
        List { 
            head: Some(Arc::new( 
                Node {
                    element, 
                    next: tail.head.clone()
                }
            ))
        }
    }

    pub fn from_value(element: T) -> List<T> {
        List { 
            head: Some(
                Arc::new( Node { element, next: None } )
            )
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn popped_front(&self) -> List<T> {
        if self.head.is_none() {
            panic!("You can't pop an empty list!");
        }
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn tail(&self) -> List<T> {
        self.popped_front()
    }

    pub fn head_tail(&self) -> (Option<&T>, List<T>) {
        (self.front(), self.tail())
    }

    pub fn pushed_front(&self, value: T) -> List<T> {
        List::cons(value, self)
    }

    pub fn reversed(&self) -> Self
    where
        T: Clone,
    {
        reverse(self)
    }

     pub fn iter(&self) -> Iter<'_, T> {
         Iter { next: self.head.as_ref().map(|node| &**node) }
     }

}

#[macro_export]
macro_rules! list_synced {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_list = List::empty();
            $(
                temp_list = temp_list.pushed_front($x);
             )*
            temp_list
        }
    };
}

impl<'a, T> IntoIterator for &'a List<T> 
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

impl<T> Clone for List<T> 
{
    fn clone(&self) -> Self {
        List { head: self.head.clone() }
    }
}

// impl<T: fmt::Debug> fmt::Debug for List<T> {}
impl<T> fmt::Display for List<T> 
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "list [")?;
        for x in self {
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> PartialEq for List<T> 
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip_longest(other.iter()).all(|x| matches!(x, EitherOrBoth::Both(a, b) if a == b))
    }
}

pub fn filter<T: Copy>(
    p: impl FnOnce(&T) -> bool + Copy, 
    list: &List<T>
) -> List<T> {
    match list.front() {
        Some(head) => {
            let tail = filter(p, &list.popped_front());
            if p(head) {
                List::cons(*head, &tail)
            } else {
                tail
            }
        },
        None => List::empty()
        
    }
} 

pub fn reverse<T: Clone>(list: &List<T>) -> List<T> {
    foldl(
        |acc: List<T>, v: &T| List::cons(v.clone(), &acc), 
        List::empty(),
        list
    )
}

pub fn fmap<U, T>(f: impl Fn(&T) -> U, list: &List<T>) -> List<U> 
where
    T: Clone,
    U: Clone,
{
    let mut result = List::<U>::empty();
    for x in list {
        result = result.pushed_front(f(&x));
    }
    result
    // match list.front() {
    //     None => List::<U>::empty(),
    //     Some(head) => List::cons(f(*head), &fmap(f, &list.popped_front()))
    // }
}

pub fn foldl<U, T>(f: impl FnOnce(U, &T) -> U + Copy, acc: U, list: &List<T>) -> U 
where
    T: Clone,
{
    match list.front() {
        None => acc,
        Some(head) => foldl(f, f(acc, head), &list.popped_front())
    }
}

pub fn foldr<U, T>(f: impl FnOnce(&T, U) -> U + Copy, acc: U, list: &List<T>) -> U 
where
    T: Clone,
{
    match list.front() {
        None => acc,
        Some(head) => f(head, foldr(f, acc, &list.popped_front()))
    }
}

pub fn for_each<T>(list: &List<T>, mut f: impl FnMut(&T) + Copy) {
    let mut node = &list.head;
    loop {
        match node {
            None => break,
            Some(head) => {
                f(&head.element);
                node = &head.next;
            }
        };
    }
}

pub fn concat<T: Clone>(a: &List<T>, b: &List<T>) -> List<T> {
    match a.front() {
        None => b.clone(),
        Some(head) => List::cons(head.clone(), &concat(&a.popped_front(), b))
    }
}

pub fn concat_all<T: Clone>(xss: &List<List<T>>) -> List<T> {
    // let result = foldr(|xs, acc| concat(xs, &acc), List::<T>::empty(), xss);
    let mut result = List::<T>::empty();
    for xs in xss {
        for x in xs {
            result = result.pushed_front(x.clone());
        };
    };
    result
}

// List Monad
pub fn mreturn<T>(t: T) -> List<T> 
{
    List::cons(t, &List::empty())
}

pub fn mbind<A: Copy, B: Copy>(list: &List<A>, k: impl Fn(&A) -> List<B> + Copy) -> List<B> {
    let list_list = fmap(k, list);
    concat_all(&list_list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty() {
        let list = List::<i32>::empty();

        match &list.head {
            None => assert!(true),
            _ => assert!(false),
        };
        assert!(list.is_empty());
        assert_eq!(list.front(), None);
    }

    #[test]
    fn create_cons() {

        let list = List::cons(3, &List::empty());

        assert_eq!(list.front(), Some(&3));
        match &list.head {
            Some(node) => {
                assert_eq!(&node.element, &3);
                assert!(node.next.is_none());
            },
            _ => panic!("Should not be here.")
        };
        assert!(!list.is_empty());
    }

    #[test]
    fn pushed_front_creates_new_longer_list() {
        let l1 = List::empty();
        let l2 = l1.pushed_front(6.7);

        assert!(l1.is_empty());
        assert_eq!(l1.front(), None);

        assert!(!l2.is_empty());
        assert_eq!(l2.front(), Some(&6.7));
    }

    #[test]
    fn popped_front_returns_tail() {
        let l1 = List::empty();
        let l2 = List::cons(3, &l1);
        let l3 = List::cons(4, &l2);

        assert_eq!(l3.front(), Some(&4));
        let l4 = l3.popped_front();
        assert_eq!(l4.front(), Some(&3));
    }

    #[test]
    fn list_macro_creates_list_in_reversed_order() {
        let l1 = list_synced!(1);
        assert_eq!(l1.front(), Some(&1));
        assert!(l1.popped_front().is_empty());

        let l2 = list_synced!(1, 2);
        assert_eq!(l2.front(), Some(&2));
        assert_eq!(l2.popped_front().front(), Some(&1));
        assert!(l2.popped_front().popped_front().is_empty());
    }

    #[test]
    fn filter_creates_new_list_with_fn_predicate() {
        fn even(v: &i32) -> bool {
            v % 2 == 0
        }

        let list = list_synced!(4, 3, 2, 1);

        let evens = filter(even, &list);

        assert_eq!(evens, list_synced!(4, 2));

    }

    #[test]
    fn test_partial_eq() {
        let l1 = list_synced!(1, 2, 3);

        assert_eq!(l1, l1);
        assert_eq!(List::<i32>::empty(), List::<i32>::empty());
        assert_eq!(
            list_synced!(5, 7, 0), 
            list_synced!(5, 7, 0));
    }

    #[test]
    fn fmap_creates_new_list_with_fn_function() {
        fn double(v: &i32) -> i32 {
            v * 2
        }

        let list = list_synced!(4, 3, 2, 1);

        let doubles = fmap(double, &list);

        assert_eq!(doubles, list_synced!(2, 4, 6, 8));

    }

    #[test]
    fn sum_w_foldl_and_foldr_are_equal() {
        fn sum(a: i32, b: &i32) -> i32 {
            a + b
        }

        let list = list_synced!(4, 3, 2, 1);

        assert_eq!(
            foldl(sum, 0, &list), 
            foldr(|a, b| a+b, 0, &list)
        );

    }

    #[test]
    fn mreturn_creates_list() {
        let list = mreturn(3);

        assert_eq!(list, list_synced!(3));
    }
}
