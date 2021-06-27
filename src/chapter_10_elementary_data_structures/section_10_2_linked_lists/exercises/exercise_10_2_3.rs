use super::super::super::section_10_1_stacks_and_queues::extra::Queue;
use std::cell::RefCell;
use std::rc::Rc;

struct SinglyLinkedListElement<T> {
    key: T,
    next: RefCell<Option<Rc<SinglyLinkedListElement<T>>>>,
}

pub struct SinglyLinkedListQueue<T> {
    head: Option<Rc<SinglyLinkedListElement<T>>>,
    tail: Option<Rc<SinglyLinkedListElement<T>>>,
    length: usize,
}

impl<T> Drop for SinglyLinkedListQueue<T> {
    fn drop(&mut self) {
        let mut maybe_element = self.head.take();

        while let Some(element) = maybe_element {
            maybe_element = element.next.borrow_mut().take();
        }
    }
}

impl<T> Default for SinglyLinkedListQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SinglyLinkedListQueue<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

impl<T> Queue<T> for SinglyLinkedListQueue<T> {
    fn empty(&self) -> bool {
        self.length == 0
    }

    fn enqueue(&mut self, x: T) {
        let new_element = Some(Rc::new(SinglyLinkedListElement {
            key: x,
            next: RefCell::new(None),
        }));

        if let Some(tail_element) = self.tail.take() {
            *tail_element.next.borrow_mut() = new_element.clone();
        } else {
            self.head = new_element.clone();
        }

        self.tail = new_element;

        self.length += 1;
    }

    fn dequeue(&mut self) -> T {
        let head_element = self.head.take().unwrap();

        if let Some(next_element) = head_element.next.borrow_mut().take() {
            self.head = Some(next_element);
        } else {
            self.head = None;
            self.tail = None;
        }

        self.length -= 1;

        Rc::try_unwrap(head_element).ok().unwrap().key
    }

    fn length(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_10_1_stacks_and_queues::tests::run_queue_test_cases;
    use super::SinglyLinkedListQueue;

    #[test]
    fn test_singly_linked_list_queue() {
        run_queue_test_cases(SinglyLinkedListQueue::new);
    }
}
