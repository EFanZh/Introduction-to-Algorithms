#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Handle(usize);

pub const NIL: Handle = Handle(usize::max_value());

fn fix_doubly_linked_list(prev: &mut [Handle], next: &mut [Handle], mut head: Handle) {
    if head != NIL {
        prev[head.0] = NIL;

        loop {
            let head_next = next[head.0];

            if head_next != NIL {
                prev[head_next.0] = head;

                head = head_next;
            } else {
                break;
            }
        }
    }
}

pub fn compactify_list<T>(
    key: &mut [T],
    prev: &mut [Handle],
    next: &mut [Handle],
    mut l: Handle,
    mut f: Handle,
) -> (Handle, Handle) {
    // First, we make sure that the free list is also correctly doubly linked.

    fix_doubly_linked_list(prev, next, f);

    // Then, we can safely reorder the elements without worrying about possibly invalid `prev` values.

    let mut element = l;
    let mut i = 0;

    while element != NIL {
        // Swap the location of `l2` and `i`.

        if element.0 == i {
            element = next[element.0];
        } else {
            // Swap the contents of two elements.

            key.swap(element.0, i);
            prev.swap(element.0, i);
            next.swap(element.0, i);

            // Fix pointers that points to the swapped elements.

            let element_next = next[element.0];
            let element_prev = prev[element.0];
            let i_prev = prev[i];
            let i_next = next[i];

            if element_prev != NIL {
                next[element_prev.0] = element;
            } else {
                f = element // This is the new head of the free list.
            }

            if element_next != NIL {
                prev[element_next.0] = element;
            }

            if i_prev != NIL {
                next[i_prev.0] = Handle(i);
            } else {
                l = Handle(i);
            }

            if i_next != NIL {
                prev[i_next.0] = Handle(i);
            }

            element = i_next;
        }

        i += 1;
    }

    (l, f)
}

#[cfg(test)]
mod tests {
    use super::{compactify_list, Handle, NIL};

    fn check_doubly_linked_list(prev: &[Handle], next: &[Handle], mut head: Handle) {
        if head != NIL {
            assert_eq!(prev[head.0], NIL);

            loop {
                let head_next = next[head.0];

                if head_next != NIL {
                    assert_eq!(prev[head_next.0], head);

                    head = head_next;
                } else {
                    break;
                }
            }
        }
    }

    fn singly_linked_list_to_vec<T: Clone>(key: &[T], next: &[Handle], mut head: Handle) -> Vec<T> {
        let mut result = Vec::new();

        while head != NIL {
            result.push(key[head.0].clone());

            head = next[head.0];
        }

        result
    }

    fn collect_singly_linked_list_handles_sorted(next: &[Handle], mut head: Handle) -> Vec<usize> {
        let mut result = Vec::new();

        while head != NIL {
            result.push(head.0);

            head = next[head.0];
        }

        result.sort_unstable();

        result
    }

    fn run_single_test(key: &mut [i32], prev: &mut [Handle], next: &mut [Handle], l: Handle, f: Handle) {
        check_doubly_linked_list(prev, next, l);

        let original_elements = singly_linked_list_to_vec(key, next, l);
        let (new_l, new_f) = compactify_list(key, prev, next, l, f);

        // Check doubly linked list property.

        check_doubly_linked_list(prev, next, new_l);

        // Check that the order of elements does not change.

        let new_elements = singly_linked_list_to_vec(key, next, new_l);

        assert_eq!(original_elements, new_elements);

        // Check the new list is compact.

        assert!(collect_singly_linked_list_handles_sorted(next, new_l)
            .into_iter()
            .eq(0..new_elements.len()));

        assert!(collect_singly_linked_list_handles_sorted(next, new_f)
            .into_iter()
            .eq(new_elements.len()..key.len()));
    }

    #[test]
    fn test_compactify_list_empty() {
        let key = &mut [];
        let prev = &mut [];
        let next = &mut [];
        let l = NIL;
        let f = NIL;

        run_single_test(key, prev, next, l, f);
    }

    #[test]
    fn test_compactify_all_free() {
        let key = &mut [0, 0, 0];
        let prev = &mut [Handle(777), Handle(888), Handle(999)];
        let next = &mut [Handle(1), Handle(2), NIL];
        let l = NIL;
        let f = Handle(0);

        run_single_test(key, prev, next, l, f);
    }

    #[test]
    fn test_compactify_all_used() {
        let key = &mut [0, 0, 0];
        let prev = &mut [NIL, Handle(0), Handle(1)];
        let next = &mut [Handle(1), Handle(2), NIL];
        let l = Handle(0);
        let f = NIL;

        run_single_test(key, prev, next, l, f);
    }

    #[test]
    fn test_compactify_interleave() {
        let key = &mut [2, 3, 5, 7];
        let prev = &mut [NIL, Handle(789), Handle(0), Handle(456)];
        let next = &mut [Handle(2), Handle(3), NIL, NIL];
        let l = Handle(0);
        let f = Handle(1);

        run_single_test(key, prev, next, l, f);
    }

    #[test]
    fn test_compactify_interleave_2() {
        let key = &mut [2, 3, 5, 7];
        let prev = &mut [Handle(789), NIL, Handle(456), Handle(1)];
        let next = &mut [Handle(2), Handle(3), NIL, NIL];
        let l = Handle(1);
        let f = Handle(0);

        run_single_test(key, prev, next, l, f);
    }
}