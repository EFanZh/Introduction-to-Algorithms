use super::super::extra::{Queue, Stack};
use super::super::ArrayQueue;
use std::mem;

pub struct ArrayQueueStack<T> {
    queue: ArrayQueue<T>,
    temp_queue: ArrayQueue<T>,
}

impl<T> Default for ArrayQueueStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArrayQueueStack<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: ArrayQueue::new(),
            temp_queue: ArrayQueue::new(),
        }
    }
}

impl<T> Stack<T> for ArrayQueueStack<T> {
    fn push(&mut self, x: T) {
        self.queue.enqueue(x);
    }

    fn pop(&mut self) -> T {
        for _ in 1..self.queue.length() {
            self.temp_queue.enqueue(self.queue.dequeue());
        }

        let result = self.queue.dequeue();

        mem::swap(&mut self.queue, &mut self.temp_queue);

        result
    }

    fn empty(&self) -> bool {
        self.queue.empty()
    }

    fn length(&self) -> usize {
        self.queue.length()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_stack_test_cases;
    use super::ArrayQueueStack;

    #[test]
    fn test_array_queue_stack() {
        run_stack_test_cases(ArrayQueueStack::new);
    }
}
