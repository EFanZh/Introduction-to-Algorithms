use super::super::super::super::chapter_10_elementary_data_structures::section_10_1_stacks_and_queues::extra::{
    Queue, Stack,
};
use super::super::super::super::utilities::KeyValuePair;
use super::super::extra::{MaxPriorityQueue, MinPriorityQueue, VecMaxPriorityQueue, VecMinPriorityQueue};

pub struct FifoQueue<T> {
    q: VecMinPriorityQueue<KeyValuePair<usize, T>>,
    next_key: usize,
}

impl<T: Ord> Default for FifoQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FifoQueue<T> {
    pub fn new() -> Self {
        Self {
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

    fn length(&self) -> usize {
        self.q.length()
    }
}

pub struct LifoStack<T> {
    q: VecMaxPriorityQueue<KeyValuePair<usize, T>>,
    next_key: usize,
}

impl<T: Ord> Default for LifoStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> LifoStack<T> {
    pub fn new() -> Self {
        Self {
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

    fn length(&self) -> usize {
        self.q.length()
    }
}

#[cfg(test)]
mod tests {
    use super::{FifoQueue, LifoStack};

    use super::super::super::super::super::chapter_10_elementary_data_structures::section_10_1_stacks_and_queues::tests::{
        run_queue_test_cases,
        run_stack_test_cases
    };

    #[test]
    fn test_fifo_queue() {
        run_queue_test_cases(FifoQueue::new);
    }

    #[test]
    fn test_lifo_stack() {
        run_stack_test_cases(LifoStack::new);
    }
}
