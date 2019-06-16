use extra::{Queue, Stack};
use std::collections::VecDeque;

pub mod exercises;
pub mod extra;

pub struct ArrayStack<T> {
    storage: Vec<T>,
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        ArrayStack { storage: Vec::new() }
    }
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ArrayStack {
            storage: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Stack<T> for ArrayStack<T> {
    fn empty(&self) -> bool {
        self.storage.is_empty()
    }

    fn push(&mut self, x: T) {
        self.storage.push(x)
    }

    fn pop(&mut self) -> T {
        self.storage.pop().unwrap()
    }
}

pub struct ArrayQueue<T> {
    storage: VecDeque<T>,
}

impl<T> Default for ArrayQueue<T> {
    fn default() -> Self {
        ArrayQueue {
            storage: VecDeque::new(),
        }
    }
}

impl<T> ArrayQueue<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ArrayQueue {
            storage: VecDeque::with_capacity(capacity),
        }
    }
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn empty(&self) -> bool {
        self.storage.is_empty()
    }

    fn enqueue(&mut self, x: T) {
        self.storage.push_back(x);
    }

    fn dequeue(&mut self) -> T {
        self.storage.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::extra::{Queue, Stack};
    use super::{ArrayQueue, ArrayStack};

    #[test]
    fn test_array_stack_empty() {
        let mut s = ArrayStack::new();

        assert!(s.empty());

        s.push(1);

        assert!(!s.empty());

        s.pop();

        assert!(s.empty());
    }

    #[test]
    fn test_array_stack_behavior() {
        let mut s = ArrayStack::new();

        s.push(1);
        s.push(2);
        s.push(3);

        assert_eq!(s.pop(), 3);
        assert_eq!(s.pop(), 2);

        s.push(4);

        assert_eq!(s.pop(), 4);
        assert_eq!(s.pop(), 1);
    }

    #[test]
    fn test_array_queue_empty() {
        let mut q = ArrayQueue::new();

        assert!(q.empty());

        q.enqueue(1);

        assert!(!q.empty());

        q.dequeue();

        assert!(q.empty());
    }

    #[test]
    fn test_array_queue_behavior() {
        let mut q = ArrayQueue::new();

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.dequeue(), 2);

        q.enqueue(4);

        assert_eq!(q.dequeue(), 3);
        assert_eq!(q.dequeue(), 4);
    }
}
