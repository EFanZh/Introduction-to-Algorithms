use super::super::extra::{Queue, Stack};
use super::super::ArrayQueue;
use std::mem::swap;

pub struct ArrayQueueStack<T> {
    bottom: ArrayQueue<T>,
    top: ArrayQueue<T>,
}

impl<T> Default for ArrayQueueStack<T> {
    fn default() -> Self {
        ArrayQueueStack {
            bottom: Default::default(),
            top: Default::default(), // Invariant: if the stack is non-empty, `top` is non-empty.
        }
    }
}

impl<T> ArrayQueueStack<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Stack<T> for ArrayQueueStack<T> {
    fn push(&mut self, x: T) {
        while !self.top.empty() {
            self.bottom.enqueue(self.top.dequeue())
        }

        self.top.enqueue(x);
    }

    fn pop(&mut self) -> T {
        for _ in 1..self.bottom.length() {
            self.top.enqueue(self.bottom.dequeue());
        }

        swap(&mut self.bottom, &mut self.top);

        self.bottom.dequeue()
    }

    fn empty(&self) -> bool {
        self.top.empty()
    }

    fn length(&self) -> usize {
        self.bottom.length() + self.top.length()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_stack_tests;
    use super::ArrayQueueStack;

    #[test]
    fn test_array_queue_stack() {
        run_stack_tests(ArrayQueueStack::new);
    }
}
