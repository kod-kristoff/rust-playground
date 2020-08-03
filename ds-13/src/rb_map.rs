use std::cmp::Ordering;
use crate::rb_tree::{RBTree};

#[derive(Debug)]
pub struct KeyValue<K, V>(K, V);

pub struct RBMap<K, V>(RBTree<KeyValue<K, V>>);

impl<K, V> Copy for KeyValue<K, V> 
where
    K: Copy,
    V: Copy,
{}

impl<K, V> Clone for KeyValue<K, V> 
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        KeyValue(self.0.clone(), self.1.clone())
    }
}

impl<K, V> PartialOrd for KeyValue<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K, V> PartialEq for KeyValue<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K, V> PartialOrd<K> for KeyValue<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl<K, V> PartialEq<K> for KeyValue<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &K) -> bool {
        self.0 == *other
    }
}



//pub struct RBMap<K, V> {
//    root: Rc<RBNode<(K, V)>>,
//}

impl<K, V> RBMap<K, V>
where
    K: Clone + PartialOrd,
    V: Clone,
{
    pub fn new() -> Self {
        RBMap( RBTree::<KeyValue<K, V>>::new() )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.0.contains(k)
    }

    pub fn inserted(&self, k: K, v: V) -> Self {
        RBMap( self.0.inserted(KeyValue(k, v)) )
    }

    pub fn inserted_or_replaced(&self, k: K, v: V) -> Self {
        RBMap(
            self.0.inserted_or_replaced(KeyValue(k, v))
        )
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        match self.0.get(k) {
            None => None,
            Some(kv) => Some(&kv.1),
        }
    }

    pub fn get_or_default<'a>(&'a self, k: &K, default: &'a V) -> &'a V {
        match self.0.get(k) {
            None => default,
            Some(kv) => &kv.1,
        }
    }

    pub fn get_key_value(&self, k: &K) -> Option<(&K, &V)> {
        match self.0.get(k) {
            None => None,
            Some(kv) => Some((&kv.0, &kv.1))
        }
    }
}

impl<K, V> Clone for RBMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        RBMap(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_creates_map() {
        let map = RBMap::<i32, &str>::new();

        assert!(map.is_empty());
        assert!(!map.contains_key(&5));
    }

    #[test]
    fn inserted_returns_larger_tree() {
        let m1 = RBMap::new();

        let m = m1.inserted(5, "b");

        assert!(!m.is_empty());
        assert!(m.contains_key(&5));
        assert!(!m.contains_key(&6));

        assert_eq!(m.get_key_value(&5), Some((&5, &"b")));
        assert_eq!(m.get_key_value(&6), None);
        assert_eq!(m.get(&5), Some(&"b"));
        assert_eq!(m.get(&6), None);
    }

    #[test]
    fn get_or_default() {
        let m1 = RBMap::new();

        let m = m1.inserted_or_replaced("g", 5);

        assert_eq!(m1.get_or_default(&"g", &0), &0);
        assert_eq!(m.get_or_default(&"g", &0), &5);
    }
} // mod tests
