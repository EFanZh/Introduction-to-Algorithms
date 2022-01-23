use super::exercises::exercise_6_5_3;

pub trait MaxPriorityQueue<T> {
    fn empty(&self) -> bool;
    fn insert(&mut self, x: T);
    fn maximum(&self) -> &T;
    fn extract_max(&mut self) -> T;
    fn length(&self) -> usize;
}

pub trait MinPriorityQueue<T> {
    fn empty(&self) -> bool;
    fn insert(&mut self, x: T);
    fn minimum(&self) -> &T;
    fn extract_min(&mut self) -> T;
    fn length(&self) -> usize;
}

pub struct VecMaxPriorityQueue<T> {
    a: Vec<T>,
}

impl<T: Ord> Default for VecMaxPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> VecMaxPriorityQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }
}

impl<T: Ord> MaxPriorityQueue<T> for VecMaxPriorityQueue<T> {
    fn empty(&self) -> bool {
        self.a.is_empty()
    }

    fn insert(&mut self, x: T) {
        super::max_heap_insert(&mut self.a, x);
    }

    fn maximum(&self) -> &T {
        super::heap_maximum(&self.a)
    }

    fn extract_max(&mut self) -> T {
        super::heap_extract_max(&mut self.a)
    }

    fn length(&self) -> usize {
        self.a.len()
    }
}

pub struct VecMinPriorityQueue<T> {
    a: Vec<T>,
}

impl<T: Ord> Default for VecMinPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> VecMinPriorityQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }
}

impl<T: Ord> MinPriorityQueue<T> for VecMinPriorityQueue<T> {
    fn empty(&self) -> bool {
        self.a.is_empty()
    }

    fn insert(&mut self, x: T) {
        exercise_6_5_3::min_heap_insert(&mut self.a, x);
    }

    fn minimum(&self) -> &T {
        exercise_6_5_3::heap_minimum(&self.a)
    }

    fn extract_min(&mut self) -> T {
        exercise_6_5_3::heap_extract_min(&mut self.a)
    }

    fn length(&self) -> usize {
        self.a.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{MaxPriorityQueue, MinPriorityQueue, VecMaxPriorityQueue, VecMinPriorityQueue};

    #[test]
    fn test_vec_max_priority_queue_empty() {
        let mut q = VecMaxPriorityQueue::new();

        assert!(q.empty());

        q.insert(4);

        assert!(!q.empty());

        q.insert(5);

        assert!(!q.empty());

        q.extract_max();

        assert!(!q.empty());

        q.extract_max();

        assert!(q.empty());
    }

    #[test]
    fn test_vec_max_priority_queue_insert() {
        let mut q = VecMaxPriorityQueue::new();

        q.insert(7);

        assert_eq!(*q.maximum(), 7);

        q.insert(4);

        assert_eq!(*q.maximum(), 7);

        q.insert(8);

        assert_eq!(*q.maximum(), 8);
    }

    #[test]
    fn test_vec_max_priority_extract_max() {
        let mut q = VecMaxPriorityQueue::new();

        q.insert(2);
        q.insert(3);
        q.insert(7);
        q.insert(6);
        q.insert(1);

        assert_eq!(q.extract_max(), 7);
        assert_eq!(q.extract_max(), 6);
        assert_eq!(q.extract_max(), 3);
        assert_eq!(q.extract_max(), 2);
        assert_eq!(q.extract_max(), 1);
    }

    #[test]
    fn test_vec_min_priority_queue_empty() {
        let mut q = VecMinPriorityQueue::new();

        assert!(q.empty());

        q.insert(4);

        assert!(!q.empty());

        q.insert(5);

        assert!(!q.empty());

        q.extract_min();

        assert!(!q.empty());

        q.extract_min();

        assert!(q.empty());
    }

    #[test]
    fn test_vec_min_priority_queue_insert() {
        let mut q = VecMinPriorityQueue::new();

        q.insert(7);

        assert_eq!(*q.minimum(), 7);

        q.insert(4);

        assert_eq!(*q.minimum(), 4);

        q.insert(8);

        assert_eq!(*q.minimum(), 4);
    }

    #[test]
    fn test_vec_min_priority_extract_min() {
        let mut q = VecMinPriorityQueue::new();

        q.insert(2);
        q.insert(3);
        q.insert(7);
        q.insert(6);
        q.insert(1);

        assert_eq!(q.extract_min(), 1);
        assert_eq!(q.extract_min(), 2);
        assert_eq!(q.extract_min(), 3);
        assert_eq!(q.extract_min(), 6);
        assert_eq!(q.extract_min(), 7);
    }
}
