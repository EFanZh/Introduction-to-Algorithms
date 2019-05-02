use super::super::super::super::section_10_elementary_data_structures::section_10_1_stacks_and_queues::extra::{
    Queue, Stack,
};
use super::super::extra::{MaxPriorityQueue, MinPriorityQueue, VecMaxPriorityQueue, VecMinPriorityQueue};
use std::cmp::Ordering;

struct KeyValuePair<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValuePair<K, V> {
    fn new(key: K, value: V) -> Self {
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

pub struct FifoQueue<T> {
    q: VecMinPriorityQueue<KeyValuePair<usize, T>>,
    next_key: usize,
}

impl<T> Default for FifoQueue<T> {
    fn default() -> Self {
        FifoQueue {
            q: VecMinPriorityQueue::new(),
            next_key: 0,
        }
    }
}

impl<T: Ord> FifoQueue<T> {
    pub fn new() -> Self {
        FifoQueue {
            q: VecMinPriorityQueue::new(),
            next_key: 0,
        }
    }
}

impl<T: Ord> Queue<T> for FifoQueue<T> {
    fn empty(&self) -> bool {
        self.q.empty()
    }

    fn enqueue(&mut self, x: T) {
        self.q.insert(KeyValuePair::new(self.next_key, x));

        self.next_key += 1;
    }

    fn dequeue(&mut self) -> T {
        self.q.extract_min().value
    }
}

pub struct LifoStack<T> {
    q: VecMaxPriorityQueue<KeyValuePair<usize, T>>,
    next_key: usize,
}

impl<T> Default for LifoStack<T> {
    fn default() -> Self {
        LifoStack {
            q: VecMaxPriorityQueue::new(),
            next_key: 0,
        }
    }
}

impl<T: Ord> LifoStack<T> {
    pub fn new() -> Self {
        LifoStack {
            q: VecMaxPriorityQueue::new(),
            next_key: 0,
        }
    }
}

impl<T: Ord> Stack<T> for LifoStack<T> {
    fn empty(&self) -> bool {
        self.q.empty()
    }

    fn push(&mut self, x: T) {
        self.q.insert(KeyValuePair::new(self.next_key, x));

        self.next_key += 1;
    }

    fn pop(&mut self) -> T {
        self.q.extract_max().value
    }
}

#[cfg(test)]
mod tests {
    use super::{FifoQueue, LifoStack, Queue, Stack};

    #[test]
    fn test_fifo_queue_empty() {
        let mut q = FifoQueue::new();

        assert!(q.empty());

        q.enqueue(4);

        assert!(!q.empty());

        q.enqueue(5);

        assert!(!q.empty());

        q.dequeue();

        assert!(!q.empty());

        q.dequeue();

        assert!(q.empty());
    }

    #[test]
    fn test_fifo_queue_fifo() {
        let mut q = FifoQueue::new();

        q.enqueue(2);

        assert_eq!(q.dequeue(), 2);

        q.enqueue(3);
        q.enqueue(5);

        assert_eq!(q.dequeue(), 3);
        assert_eq!(q.dequeue(), 5);

        q.enqueue(7);
        q.enqueue(11);
        q.enqueue(13);

        assert_eq!(q.dequeue(), 7);
        assert_eq!(q.dequeue(), 11);

        q.enqueue(17);

        assert_eq!(q.dequeue(), 13);
        assert_eq!(q.dequeue(), 17);
    }

    #[test]
    fn test_lifo_stack_empty() {
        let mut q = LifoStack::new();

        assert!(q.empty());

        q.push(4);

        assert!(!q.empty());

        q.push(5);

        assert!(!q.empty());

        q.pop();

        assert!(!q.empty());

        q.pop();

        assert!(q.empty());
    }

    #[test]
    fn test_lifo_stack_lifo() {
        let mut q = LifoStack::new();

        q.push(2);

        assert_eq!(q.pop(), 2);

        q.push(3);
        q.push(5);

        assert_eq!(q.pop(), 5);
        assert_eq!(q.pop(), 3);

        q.push(7);
        q.push(11);
        q.push(13);

        assert_eq!(q.pop(), 13);
        assert_eq!(q.pop(), 11);

        q.push(17);

        assert_eq!(q.pop(), 17);
        assert_eq!(q.pop(), 7);
    }
}
