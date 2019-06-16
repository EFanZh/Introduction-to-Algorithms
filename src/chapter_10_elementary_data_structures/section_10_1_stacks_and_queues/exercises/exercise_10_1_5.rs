use std::iter;

pub struct FixedSizeArrayDeque<T> {
    // This `Option` here is why I don’t want to implement the stack and queue using raw arrays.
    storage: Box<[Option<T>]>,
    head: usize,
    tail: usize,
}

impl<T> FixedSizeArrayDeque<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: iter::repeat_with(|| None).take(capacity + 1).collect(),
            head: 0,
            tail: 0,
        }
    }

    fn next_index(&self, index: usize) -> usize {
        let result = index + 1;

        if result == self.storage.len() {
            0
        } else {
            result
        }
    }

    fn previous_index(&self, index: usize) -> usize {
        if index == 0 {
            self.storage.len() - 1
        } else {
            index - 1
        }
    }

    fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    fn is_full(&self) -> bool {
        self.next_index(self.tail) == self.head
    }

    pub fn push_back(&mut self, value: T) {
        assert!(!self.is_full());

        self.storage[self.tail] = Some(value);
        self.tail = self.next_index(self.tail);
    }

    pub fn pop_back(&mut self) -> T {
        assert!(!self.is_empty());

        let index = self.previous_index(self.tail);
        let result = self.storage[index].take().unwrap();

        self.tail = index;

        result
    }

    pub fn push_front(&mut self, value: T) {
        assert!(!self.is_full());

        let index = self.previous_index(self.head);

        self.storage[index] = Some(value);
        self.head = index;
    }

    pub fn pop_front(&mut self) -> T {
        assert!(!self.is_empty());

        let result = self.storage[self.head].take().unwrap();

        self.head = self.next_index(self.head);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::FixedSizeArrayDeque;

    #[test]
    fn test_array_deque_behavior_length_1() {
        let mut deque = FixedSizeArrayDeque::with_capacity(1);

        deque.push_back(1);

        assert_eq!(deque.pop_back(), 1);

        deque.push_back(1);

        assert_eq!(deque.pop_front(), 1);

        deque.push_front(1);

        assert_eq!(deque.pop_front(), 1);

        deque.push_front(1);

        assert_eq!(deque.pop_back(), 1);
    }

    #[test]
    fn test_array_deque_behavior_normal() {
        let mut deque = FixedSizeArrayDeque::with_capacity(10);

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);

        assert_eq!(deque.pop_back(), 4);
        assert_eq!(deque.pop_back(), 3);
        assert_eq!(deque.pop_front(), 1);
        assert_eq!(deque.pop_front(), 2);

        deque.push_front(5);
        deque.push_front(6);
        deque.push_front(7);
        deque.push_front(8);

        assert_eq!(deque.pop_back(), 5);
        assert_eq!(deque.pop_back(), 6);
        assert_eq!(deque.pop_front(), 8);
        assert_eq!(deque.pop_front(), 7);
    }
}
