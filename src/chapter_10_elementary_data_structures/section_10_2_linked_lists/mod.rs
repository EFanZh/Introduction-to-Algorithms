use std::cell::{Ref, RefCell};
use std::iter;
use std::rc::Rc;

pub mod exercises;

struct DoublyLinkedListElementContent<T> {
    key: T,
    next: Option<DoublyLinkedListElement<T>>,
    prev: Option<DoublyLinkedListElement<T>>,
}

pub struct DoublyLinkedListElement<T>(Rc<RefCell<DoublyLinkedListElementContent<T>>>);

impl<T> DoublyLinkedListElement<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(DoublyLinkedListElementContent {
            key: value,
            next: None,
            prev: None,
        })))
    }

    #[must_use]
    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |x| &x.key)
    }
}

impl<T> Clone for DoublyLinkedListElement<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<DoublyLinkedListElement<T>>,
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
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

impl<T> DoublyLinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { head: None }
    }

    // List-Search(L, k)
    //
    // 1  x = L.head
    // 2  while x ≠ nil and x.key ≠ k
    // 3      x = x.next
    // 4  return x

    pub fn search<U>(&self, k: &U) -> Option<DoublyLinkedListElement<T>>
    where
        T: PartialEq<U>,
    {
        self.iter().find(|element| *element.borrow() == *k)
    }

    // List-Insert(L, x)
    //
    // 1  x.next = L.head
    // 2  if L.head ≠ nil
    // 3      L.head.prev = x
    // 4  L.head = x
    // 5  x.prev = nil

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

    // List-Delete(L, x)
    //
    // 1  if x.prev ≠ nil
    // 2      x.prev.next = x.next
    // 3  else L.head = x.next
    // 4  if x.next ≠ nil
    // 5      x.next.prev = x.prev

    pub fn delete(&mut self, x: &DoublyLinkedListElement<T>) {
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

    fn iter(&self) -> impl Iterator<Item = DoublyLinkedListElement<T>> {
        iter::successors(self.head.clone(), |element| element.0.borrow().next.clone())
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
        list.iter().map(|x| *x.borrow()).collect()
    }

    fn run_tests<I: IntoIterator<Item = DoublyLinkedListOperation<i32>>>(operations: I) {
        use DoublyLinkedListOperation::{Delete, Insert, Inspect, RawInsert, Search};

        let mut list = DoublyLinkedList::new();

        for operation in operations {
            match operation {
                Search(value, found) => {
                    let result = list.search(&value);

                    if found {
                        assert_eq!(result.map(|item| *item.borrow()), Some(value));
                    } else {
                        assert!(result.is_none());
                    }
                }
                Insert(value) => list.insert(DoublyLinkedListElement::new(value)),
                RawInsert(value) => list.insert(value),
                Delete(value) => list.delete(&value),
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
