use super::super::extra::{Queue, Stack};
use super::super::ArrayStack;

pub struct ArrayStackQueue<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T> Default for ArrayStackQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArrayStackQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }
}

impl<T> Queue<T> for ArrayStackQueue<T> {
    fn enqueue(&mut self, x: T) {
        self.back.push(x);
    }

    fn dequeue(&mut self) -> T {
        if self.front.empty() {
            while !self.back.empty() {
                self.front.push(self.back.pop());
            }
        }

        self.front.pop()
    }

    fn empty(&self) -> bool {
        self.front.empty() && self.back.empty()
    }

    fn length(&self) -> usize {
        self.front.length() + self.back.length()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests;
    use super::ArrayStackQueue;

    #[test]
    fn test_array_stack_queue() {
        tests::run_queue_test_cases(ArrayStackQueue::new);
    }
}
