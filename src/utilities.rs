use std::cmp::Ordering;
use std::mem::replace;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Infinitable<T: Ord> {
    Finity(T),
    Infinity,
}

impl<T: Ord> Infinitable<T> {
    pub fn replace_with_infinity(&mut self) -> Option<T> {
        if let Infinitable::Finity(value) = replace(self, Infinitable::Infinity) {
            Some(value)
        } else {
            None
        }
    }
}

impl<T: Ord> PartialEq<T> for Infinitable<T> {
    fn eq(&self, other: &T) -> bool {
        if let Infinitable::Finity(lhs) = self {
            lhs.eq(other)
        } else {
            false
        }
    }
}

impl<T: Ord> PartialOrd<T> for Infinitable<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(match self {
            Infinitable::Finity(value) => value.cmp(other),
            Infinitable::Infinity => Ordering::Greater,
        })
    }
}

impl<T: Ord> From<T> for Infinitable<T> {
    fn from(val: T) -> Self {
        Infinitable::Finity(val)
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
