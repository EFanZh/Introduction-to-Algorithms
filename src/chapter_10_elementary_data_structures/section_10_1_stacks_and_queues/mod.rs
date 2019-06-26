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

    enum StackOperation<T> {
        Empty(bool),
        Push(T),
        Pop(T),
    }

    enum QueueOperation<T> {
        Empty(bool),
        Enqueue(T),
        Dequeue(T),
    }

    pub fn run_stack_tests<S: Stack<i32>, F: FnMut() -> S>(mut f: F) {
        use StackOperation::{Empty, Pop, Push};

        let test_cases = vec![
            vec![Empty(true)],
            vec![Empty(true), Push(3), Empty(false), Pop(3), Empty(true)],
            vec![
                Empty(true),
                Push(3),
                Push(7),
                Push(2),
                Push(4),
                Pop(4),
                Pop(2),
                Pop(7),
                Push(9),
                Pop(9),
                Pop(3),
                Empty(true),
            ],
        ];

        for test_case in test_cases {
            let mut stack = f();

            for operation in test_case {
                match operation {
                    Empty(value) => assert_eq!(stack.empty(), value),
                    Push(value) => stack.push(value),
                    Pop(value) => assert_eq!(stack.pop(), value),
                }
            }
        }
    }

    pub fn run_queue_tests<S: Queue<i32>, F: FnMut() -> S>(mut f: F) {
        use QueueOperation::{Dequeue, Empty, Enqueue};

        let test_cases = vec![
            vec![Empty(true)],
            vec![Empty(true), Enqueue(3), Empty(false), Dequeue(3), Empty(true)],
            vec![
                Empty(true),
                Enqueue(3),
                Enqueue(7),
                Enqueue(2),
                Enqueue(4),
                Dequeue(3),
                Dequeue(7),
                Dequeue(2),
                Enqueue(9),
                Dequeue(4),
                Dequeue(9),
                Empty(true),
            ],
        ];

        for test_case in test_cases {
            let mut queue = f();

            for operation in test_case {
                match operation {
                    Empty(value) => assert_eq!(queue.empty(), value),
                    Enqueue(value) => queue.enqueue(value),
                    Dequeue(value) => assert_eq!(queue.dequeue(), value),
                }
            }
        }
    }

    #[test]
    fn test_array_stack() {
        run_stack_tests(ArrayStack::new);
    }

    #[test]
    fn test_array_queue() {
        run_queue_tests(ArrayQueue::new);
    }
}
