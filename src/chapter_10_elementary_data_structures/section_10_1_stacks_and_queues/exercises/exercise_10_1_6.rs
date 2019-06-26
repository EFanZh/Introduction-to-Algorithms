use super::super::extra::{Queue, Stack};
use super::super::ArrayStack;

pub struct ArrayStackQueue<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T> Default for ArrayStackQueue<T> {
    fn default() -> Self {
        ArrayStackQueue {
            front: Default::default(), // Invariant: if the queue is non-empty, `front` is non-empty.
            back: Default::default(),
        }
    }
}

impl<T> ArrayStackQueue<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Queue<T> for ArrayStackQueue<T> {
    fn enqueue(&mut self, x: T) {
        if self.empty() {
            self.front.push(x)
        } else {
            self.back.push(x);
        }
    }

    fn dequeue(&mut self) -> T {
        let result = self.front.pop();

        if self.front.empty() {
            while !self.back.empty() {
                self.front.push(self.back.pop());
            }
        }

        result
    }

    fn empty(&self) -> bool {
        self.front.empty()
    }

    fn length(&self) -> usize {
        self.front.length() + self.back.length()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_queue_tests;
    use super::ArrayStackQueue;

    #[test]
    fn test_array_stack_queue() {
        run_queue_tests(ArrayStackQueue::new);
    }
}
