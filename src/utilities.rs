use std::cmp::Ordering;
use std::mem;
use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Infinitable<T> {
    Finite(T),
    Infinity,
}

impl<T> Infinitable<T> {
    pub fn replace_with_infinity(&mut self) -> Option<T> {
        if let Self::Finite(value) = mem::replace(self, Self::Infinity) {
            Some(value)
        } else {
            None
        }
    }
}

impl<T, U> Add<U> for Infinitable<T>
where
    T: Add<U>,
{
    type Output = Infinitable<T::Output>;

    fn add(self, rhs: U) -> Self::Output {
        if let Self::Finite(lhs) = self {
            Infinitable::Finite(lhs + rhs)
        } else {
            Infinitable::Infinity
        }
    }
}

impl<T> PartialEq<T> for Infinitable<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        if let Self::Finite(lhs) = self {
            PartialEq::eq(lhs, other)
        } else {
            false
        }
    }
}

impl<T> PartialOrd<T> for Infinitable<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        match self {
            Self::Finite(value) => PartialOrd::partial_cmp(value, other),
            Self::Infinity => Some(Ordering::Greater),
        }
    }
}

impl<T> From<T> for Infinitable<T> {
    fn from(val: T) -> Self {
        Self::Finite(val)
    }
}

pub struct KeyValuePair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K, V> KeyValuePair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
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
