use std::cmp::Ordering;
use std::{cmp::Reverse, mem::replace};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct MaxSentinel<T>(Reverse<Option<Reverse<T>>>);

impl<T> MaxSentinel<T> {
    pub fn max() -> Self {
        MaxSentinel(Reverse(None))
    }

    pub fn take_unwrap(&mut self) -> T {
        replace(&mut (self.0).0, None).unwrap().0
    }
}

impl<T> Default for MaxSentinel<T> {
    fn default() -> Self {
        MaxSentinel::max()
    }
}

impl<T> From<T> for MaxSentinel<T> {
    fn from(v: T) -> Self {
        MaxSentinel(Reverse(Some(Reverse(v))))
    }
}

pub struct KeyValuePair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> KeyValuePair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValuePair { key, value }
    }
}

impl<K: PartialEq, V> PartialEq for KeyValuePair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Eq, V> Eq for KeyValuePair<K, V> {}

impl<K: PartialOrd, V> PartialOrd for KeyValuePair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Ord for KeyValuePair<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
