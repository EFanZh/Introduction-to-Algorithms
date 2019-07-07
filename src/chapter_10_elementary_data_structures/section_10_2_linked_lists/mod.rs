use std::cell::{Ref, RefCell};
use std::mem::drop;
use std::rc::Rc;

struct DoublyLinkedListElementContent<T> {
    key: T,
    next: Option<DoublyLinkedListElement<T>>,
    prev: Option<DoublyLinkedListElement<T>>,
}

pub struct DoublyLinkedListElement<T>(Rc<RefCell<DoublyLinkedListElementContent<T>>>);

impl<T> DoublyLinkedListElement<T> {
    pub fn new(value: T) -> Self {
        DoublyLinkedListElement(Rc::new(RefCell::new(DoublyLinkedListElementContent {
            key: value,
            next: None,
            prev: None,
        })))
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |x| &x.key)
    }
}

impl<T> Clone for DoublyLinkedListElement<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<DoublyLinkedListElement<T>>,
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(element) = head {
            let mut node_ref = element.0.borrow_mut();

            node_ref.prev = None;
            head = node_ref.next.take();
        }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn search<U>(&self, k: U) -> Option<DoublyLinkedListElement<T>>
    where
        T: PartialEq<U>,
    {
        // This implementation is ugly, not sure how to improve it.

        let mut x = self.head.clone();

        while let Some(element) = x {
            let node_ref = element.0.borrow();

            if node_ref.key == k {
                drop(node_ref);

                return Some(element);
            } else {
                x = node_ref.next.clone();
            }
        }

        None
    }

    pub fn insert(&mut self, x: DoublyLinkedListElement<T>) {
        let mut x_ref = x.0.borrow_mut();

        if let Some(head_element) = self.head.take() {
            head_element.0.borrow_mut().prev = Some(x.clone());

            x_ref.next = Some(head_element);
        } else {
            x_ref.next = None;
        }

        x_ref.prev = None;

        drop(x_ref);

        self.head = Some(x);
    }

    pub fn delete(&mut self, x: DoublyLinkedListElement<T>) {
        let mut x_ref = x.0.borrow_mut();

        if let Some(x_prev) = x_ref.prev.take() {
            if let Some(x_next) = x_ref.next.take() {
                x_prev.0.borrow_mut().next = Some(x_next.clone());
                x_next.0.borrow_mut().prev = Some(x_prev);
            } else {
                x_prev.0.borrow_mut().next = None;
            }
        } else if let Some(x_next) = x_ref.next.take() {
            self.head = Some(x_next);
        } else {
            self.head = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DoublyLinkedList, DoublyLinkedListElement};

    enum DoublyLinkedListOperation<T> {
        Search(T, bool),
        Insert(T),
        RawInsert(DoublyLinkedListElement<T>),
        Delete(DoublyLinkedListElement<T>),
        Inspect(Vec<T>),
    }

    fn doubly_linked_list_to_vec<T: Copy>(list: &DoublyLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();

        let mut x = list.head.clone();

        while let Some(rc) = x {
            let node_ref = rc.0.borrow();

            result.push(node_ref.key);

            x = node_ref.next.clone();
        }

        result
    }

    fn run_tests<I: IntoIterator<Item = DoublyLinkedListOperation<i32>>>(operations: I) {
        use DoublyLinkedListOperation::{Delete, Insert, Inspect, RawInsert, Search};

        let mut list = DoublyLinkedList::new();

        for operation in operations {
            match operation {
                Search(value, found) => {
                    if let Some(result) = list.search(value) {
                        assert!(found);
                        assert_eq!(*result.borrow(), value);
                    } else {
                        assert!(!found);
                    }
                }
                Insert(value) => list.insert(DoublyLinkedListElement::new(value)),
                RawInsert(value) => list.insert(value),
                Delete(value) => list.delete(value),
                Inspect(values) => assert_eq!(doubly_linked_list_to_vec(&list), values),
            }
        }
    }

    #[test]
    fn test_doubly_linked_list() {
        use DoublyLinkedListOperation::{Delete, Insert, Inspect, RawInsert, Search};

        let test_cases = vec![
            vec![Inspect(Vec::new())],
            vec![Search(3, false)],
            vec![Insert(2), Inspect(vec![2])],
            vec![Insert(2), Insert(3), Inspect(vec![3, 2])],
            vec![Insert(2), Insert(3), Search(1, false)],
            vec![Insert(2), Insert(3), Search(2, true)],
            vec![Insert(2), Insert(3), Search(3, true)],
            {
                let node = DoublyLinkedListElement::new(7);

                vec![RawInsert(node.clone()), Delete(node), Inspect(Vec::new())]
            },
            {
                let node = DoublyLinkedListElement::new(7);

                vec![
                    RawInsert(node.clone()),
                    Insert(2),
                    Insert(5),
                    Delete(node),
                    Inspect(vec![5, 2]),
                ]
            },
            {
                let node = DoublyLinkedListElement::new(7);

                vec![
                    Insert(2),
                    RawInsert(node.clone()),
                    Insert(5),
                    Delete(node),
                    Inspect(vec![5, 2]),
                ]
            },
            {
                let node = DoublyLinkedListElement::new(7);

                vec![
                    Insert(2),
                    Insert(5),
                    RawInsert(node.clone()),
                    Delete(node),
                    Inspect(vec![5, 2]),
                ]
            },
        ];

        test_cases.into_iter().for_each(run_tests);
    }
}
