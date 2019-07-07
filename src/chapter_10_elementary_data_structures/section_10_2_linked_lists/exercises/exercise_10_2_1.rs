use std::cell::{Ref, RefCell};
use std::iter;
use std::mem;
use std::rc::Rc;

struct SinglyLinkedListElementContent<T> {
    key: T,
    next: Option<SinglyLinkedListElement<T>>,
}

pub struct SinglyLinkedListElement<T>(Rc<RefCell<SinglyLinkedListElementContent<T>>>);

impl<T> SinglyLinkedListElement<T> {
    pub fn new(value: T) -> Self {
        SinglyLinkedListElement(Rc::new(RefCell::new(SinglyLinkedListElementContent {
            key: value,
            next: None,
        })))
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |x| &x.key)
    }
}

impl<T> Clone for SinglyLinkedListElement<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<SinglyLinkedListElement<T>>,
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut maybe_element = self.head.take();

        while let Some(element) = maybe_element {
            maybe_element = element.0.borrow_mut().next.take();
        }
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn search<U>(&self, k: U) -> Option<SinglyLinkedListElement<T>>
    where
        T: PartialEq<U>,
    {
        for element in self.iter() {
            if *element.borrow() == k {
                return Some(element);
            }
        }

        None
    }

    pub fn insert(&mut self, x: SinglyLinkedListElement<T>) {
        let mut x_ref = x.0.borrow_mut();

        if let Some(head_element) = self.head.take() {
            x_ref.next = Some(head_element);
        } else {
            x_ref.next = None;
        }

        drop(x_ref);

        self.head = Some(x);
    }

    pub fn delete(&mut self, x: SinglyLinkedListElement<T>) {
        if let Some(next_rc) = {
            // Note: these two lines are necessary. Why?

            let x_ref = x.0.borrow();

            x_ref.next.clone()
        } {
            // This node is not the last node, we move the content of the next node into this node. We can do this in
            // O(1) running time. This can be done at the cost of modifying the value of the next node. If some is
            // holding an reference to the next node, the value of next node is changed silently.

            x.0.swap(&next_rc.0);

            next_rc.0.borrow_mut().next = None; // Break circular reference to self in order to free this node.
        } else {
            // This node is the last node, we have to fully traverse the list in order to find the previous node of this
            // node.

            let mut first_element = self.head.clone().unwrap();
            let maybe_second_element = first_element.0.borrow().next.clone();

            if let Some(mut second_element) = maybe_second_element {
                while let Some(next_element) = {
                    // Note: these two lines are necessary. Why?

                    let second_element_ref = second_element.0.borrow();

                    second_element_ref.next.clone()
                } {
                    first_element = second_element;
                    second_element = next_element;
                }

                assert!(Rc::ptr_eq(&second_element.0, &x.0));

                first_element.0.borrow_mut().next = None;
            } else {
                // This node only have one element.

                assert!(Rc::ptr_eq(&self.head.as_ref().unwrap().0, &x.0));

                self.head = None;
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = SinglyLinkedListElement<T>> {
        let mut maybe_element = self.head.clone();

        iter::from_fn(move || {
            if let Some(element) = &maybe_element {
                let maybe_next_element = element.0.borrow().next.clone();

                mem::replace(&mut maybe_element, maybe_next_element)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{SinglyLinkedList, SinglyLinkedListElement};

    enum SinglyLinkedListOperation<T> {
        Search(T, bool),
        Insert(T),
        RawInsert(SinglyLinkedListElement<T>),
        Delete(SinglyLinkedListElement<T>),
        Inspect(Vec<T>),
    }

    fn singly_linked_list_to_vec<T: Copy>(list: &SinglyLinkedList<T>) -> Vec<T> {
        list.iter().map(|x| *x.borrow()).collect()
    }

    fn run_tests<I: IntoIterator<Item = SinglyLinkedListOperation<i32>>>(operations: I) {
        use SinglyLinkedListOperation::{Delete, Insert, Inspect, RawInsert, Search};

        let mut list = SinglyLinkedList::new();

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
                Insert(value) => list.insert(SinglyLinkedListElement::new(value)),
                RawInsert(value) => list.insert(value),
                Delete(value) => list.delete(value),
                Inspect(values) => assert_eq!(singly_linked_list_to_vec(&list), values),
            }
        }
    }

    #[test]
    fn test_singly_linked_list() {
        use SinglyLinkedListOperation::{Delete, Insert, Inspect, RawInsert, Search};

        let test_cases = vec![
            vec![Inspect(Vec::new())],
            vec![Search(3, false)],
            vec![Insert(2), Inspect(vec![2])],
            vec![Insert(2), Insert(3), Inspect(vec![3, 2])],
            vec![Insert(2), Insert(3), Search(1, false)],
            vec![Insert(2), Insert(3), Search(2, true)],
            vec![Insert(2), Insert(3), Search(3, true)],
            {
                let node = SinglyLinkedListElement::new(7);

                vec![RawInsert(node.clone()), Delete(node), Inspect(Vec::new())]
            },
            {
                let node = SinglyLinkedListElement::new(7);

                vec![
                    RawInsert(node.clone()),
                    Insert(2),
                    Insert(5),
                    Delete(node),
                    Inspect(vec![5, 2]),
                ]
            },
            {
                let node = SinglyLinkedListElement::new(7);

                vec![
                    Insert(2),
                    RawInsert(node.clone()),
                    Insert(5),
                    Delete(node),
                    Inspect(vec![5, 2]),
                ]
            },
            {
                let node = SinglyLinkedListElement::new(7);

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
