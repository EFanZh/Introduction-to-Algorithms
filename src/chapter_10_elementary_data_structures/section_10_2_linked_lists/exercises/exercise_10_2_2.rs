use super::super::super::section_10_1_stacks_and_queues::extra::Stack;

struct SinglyLinkedListElement<T> {
    key: T,
    next: Option<Box<SinglyLinkedListElement<T>>>,
}

pub struct SinglyLinkedListStack<T> {
    head: Option<Box<SinglyLinkedListElement<T>>>,
    length: usize,
}

impl<T> Drop for SinglyLinkedListStack<T> {
    fn drop(&mut self) {
        let mut maybe_element = self.head.take();

        while let Some(mut element) = maybe_element {
            maybe_element = element.next.take();
        }
    }
}

impl<T> Default for SinglyLinkedListStack<T> {
    fn default() -> Self {
        Self { head: None, length: 0 }
    }
}

impl<T> SinglyLinkedListStack<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> Stack<T> for SinglyLinkedListStack<T> {
    fn empty(&self) -> bool {
        self.length == 0
    }

    fn push(&mut self, x: T) {
        self.head = Some(Box::new(SinglyLinkedListElement {
            key: x,
            next: self.head.take(),
        }));

        self.length += 1;
    }

    fn pop(&mut self) -> T {
        let old_head = self.head.take().unwrap();

        self.head = old_head.next;
        self.length -= 1;

        old_head.key
    }

    fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_10_1_stacks_and_queues::tests::run_stack_tests;
    use super::SinglyLinkedListStack;

    #[test]
    fn test_singly_linked_list_stack() {
        run_stack_tests(SinglyLinkedListStack::new);
    }
}
