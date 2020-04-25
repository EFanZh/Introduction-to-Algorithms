use std::iter::FromIterator;

type FixedLengthArray<T> = Box<[T]>;

pub struct AdjacencyListGraph<E> {
    adj: FixedLengthArray<FixedLengthArray<(E, usize)>>,
}

impl<E> AdjacencyListGraph<E> {
    pub fn adj(&self, i: usize) -> impl IntoIterator<Item = &(E, usize), IntoIter = impl '_ + ExactSizeIterator> {
        &*self.adj[i]
    }

    pub fn vertices(&self) -> impl IntoIterator<Item = usize, IntoIter = impl ExactSizeIterator> {
        0..self.adj.len()
    }
}

impl<E, A: IntoIterator<Item = (E, usize)>> FromIterator<A> for AdjacencyListGraph<E> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            adj: iter.into_iter().map(|item| item.into_iter().collect()).collect(),
        }
    }
}
