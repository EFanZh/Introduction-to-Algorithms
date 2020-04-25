use extra::{Queue, Stack};
use std::collections::VecDeque;

pub mod exercises;
pub mod extra;

pub struct ArrayStack<T> {
    storage: Vec<T>,
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        Self { storage: Vec::new() }
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

    fn length(&self) -> usize {
        self.storage.len()
    }
}

pub struct ArrayQueue<T> {
    storage: VecDeque<T>,
}

impl<T> Default for ArrayQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArrayQueue<T> {
    pub fn new() -> Self {
        Self {
            storage: VecDeque::new(),
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

    fn length(&self) -> usize {
        self.storage.len()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::extra::{Queue, Stack};
    use super::{ArrayQueue, ArrayStack};

    enum StackOperation<T> {
        Empty(bool),
        Push(T),
        Pop(T),
        Length(usize),
    }

    enum QueueOperation<T> {
        Empty(bool),
        Enqueue(T),
        Dequeue(T),
        Length(usize),
    }

    pub fn run_stack_test_cases<S: Stack<i32>, F: FnMut() -> S>(mut f: F) {
        use StackOperation::{Empty, Length, Pop, Push};

        let test_cases = vec![
            vec![Empty(true), Length(0)],
            vec![
                Empty(true),
                Length(0),
                Push(3),
                Empty(false),
                Length(1),
                Pop(3),
                Empty(true),
                Length(0),
            ],
            vec![
                Empty(true),
                Push(3),
                Push(7),
                Push(2),
                Push(4),
                Length(4),
                Pop(4),
                Pop(2),
                Pop(7),
                Length(1),
                Push(9),
                Length(2),
                Pop(9),
                Pop(3),
                Empty(true),
            ],
            vec![
                Empty(true),
                Push(2),
                Push(3),
                Push(5),
                Push(7),
                Push(9),
                Length(5),
                Pop(9),
                Pop(7),
                Length(3),
            ],
        ];

        for test_case in test_cases {
            let mut stack = f();

            for operation in test_case {
                match operation {
                    Empty(value) => assert_eq!(stack.empty(), value),
                    Push(value) => stack.push(value),
                    Pop(value) => assert_eq!(stack.pop(), value),
                    Length(value) => assert_eq!(stack.length(), value),
                }
            }
        }
    }

    pub fn run_queue_test_cases<S: Queue<i32>, F: FnMut() -> S>(mut f: F) {
        use QueueOperation::{Dequeue, Empty, Enqueue, Length};

        let test_cases = vec![
            vec![Empty(true), Length(0)],
            vec![
                Empty(true),
                Length(0),
                Enqueue(3),
                Empty(false),
                Dequeue(3),
                Empty(true),
                Length(0),
            ],
            vec![
                Empty(true),
                Enqueue(3),
                Enqueue(7),
                Enqueue(2),
                Enqueue(4),
                Length(4),
                Dequeue(3),
                Dequeue(7),
                Dequeue(2),
                Length(1),
                Enqueue(9),
                Length(2),
                Dequeue(4),
                Dequeue(9),
                Empty(true),
            ],
            vec![
                Empty(true),
                Enqueue(3),
                Enqueue(7),
                Enqueue(2),
                Enqueue(4),
                Enqueue(9),
                Length(5),
                Dequeue(3),
                Dequeue(7),
                Length(3),
            ],
        ];

        for test_case in test_cases {
            let mut queue = f();

            for operation in test_case {
                match operation {
                    Empty(value) => assert_eq!(queue.empty(), value),
                    Enqueue(value) => queue.enqueue(value),
                    Dequeue(value) => assert_eq!(queue.dequeue(), value),
                    Length(value) => assert_eq!(queue.length(), value),
                }
            }
        }
    }

    #[test]
    fn test_array_stack() {
        run_stack_test_cases(ArrayStack::new);
    }

    #[test]
    fn test_array_queue() {
        run_queue_test_cases(ArrayQueue::new);
    }
}
