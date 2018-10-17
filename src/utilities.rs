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
