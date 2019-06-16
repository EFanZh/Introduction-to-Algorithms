use std::collections::VecDeque;

pub struct TwoStacks<T> {
    storage: VecDeque<T>,
    left_stack_len: usize,
}

impl<T> Default for TwoStacks<T> {
    fn default() -> Self {
        Self {
            storage: VecDeque::new(),
            left_stack_len: 0,
        }
    }
}

impl<T> TwoStacks<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            storage: VecDeque::with_capacity(capacity),
            left_stack_len: 0,
        }
    }

    pub fn empty_1(&self) -> bool {
        self.left_stack_len == 0
    }

    pub fn push_1(&mut self, x: T) {
        self.storage.push_front(x);
        self.left_stack_len += 1;
    }

    pub fn pop_1(&mut self) -> T {
        assert!(!self.empty_1());

        let result = self.storage.pop_front().unwrap();

        self.left_stack_len -= 1;

        result
    }

    pub fn empty_2(&self) -> bool {
        self.storage.len() == self.left_stack_len
    }

    pub fn push_2(&mut self, x: T) {
        self.storage.push_back(x)
    }

    pub fn pop_2(&mut self) -> T {
        assert!(!self.empty_2());

        self.storage.pop_back().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::TwoStacks;

    #[test]
    fn test_two_stacks_empty() {
        let mut s = TwoStacks::new();

        assert!(s.empty_1());
        assert!(s.empty_2());

        s.push_1(1);

        assert!(!s.empty_1());
        assert!(s.empty_2());

        s.pop_1();

        assert!(s.empty_1());
        assert!(s.empty_2());

        s.push_2(2);

        assert!(s.empty_1());
        assert!(!s.empty_2());

        s.pop_2();

        assert!(s.empty_1());
        assert!(s.empty_2());
    }

    #[test]
    fn test_two_stacks_behavior() {
        let mut s = TwoStacks::new();

        s.push_1(1);
        s.push_1(2);
        s.push_1(3);
        s.push_2(10);
        s.push_2(20);
        s.push_2(30);

        assert_eq!(s.pop_1(), 3);
        assert_eq!(s.pop_1(), 2);
        assert_eq!(s.pop_2(), 30);
        assert_eq!(s.pop_2(), 20);

        s.push_1(4);
        s.push_2(40);

        assert_eq!(s.pop_1(), 4);
        assert_eq!(s.pop_1(), 1);
        assert_eq!(s.pop_2(), 40);
        assert_eq!(s.pop_2(), 10);
    }
}
