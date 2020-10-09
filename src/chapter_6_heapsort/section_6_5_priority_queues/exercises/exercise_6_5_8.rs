use super::super::super::section_6_1_heaps::parent;
use super::super::super::section_6_2_maintaining_the_heap_property::max_heapify;

pub fn heap_delete<T: Ord>(a: &mut Vec<T>, mut i: usize) {
    a.swap_remove(i);

    if i < a.len() {
        if i == 0 || a[i] <= a[parent(i)] {
            // The heap property holds on a[i]’s parent, we make sure it holds on its children.

            max_heapify(a, i);
        } else {
            // The heap property doesn’t hold on a[i]’s parent, we fix it by swapping it with its parent.

            loop {
                a.swap(i, parent(i));

                i = parent(i);

                if i == 0 || a[i] <= a[parent(i)] {
                    break;
                }
            }
        }
    }
}

pub fn heap_delete_2<T: Ord>(a: &mut Vec<T>, mut i: usize) {
    if a[a.len() - 1] <= a[i] {
        a.swap_remove(i);

        max_heapify(a, i);
    } else {
        a.swap_remove(i);

        while i > 0 && a[i] > a[parent(i)] {
            a.swap(i, parent(i));

            i = parent(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::{assign_vec, is_max_heap, loop_on_all_max_heap_test_cases};
    use super::{heap_delete, heap_delete_2};

    fn run_heap_delete_test<F: FnMut(&mut Vec<i32>, usize)>(mut f: F) {
        let mut run_deletion_test = |heap: &mut Vec<i32>, i, expected_values: &[i32]| {
            let value = heap[i]; // Save the value to be deleted.

            f(heap, i); // Delete the value.

            assert!(is_max_heap(&heap)); // Check max heap property.

            heap.push(value); // Put the deleted value back.

            heap.sort_unstable(); // Sort the heap.

            assert_eq!(heap.as_slice(), expected_values); // Check value consistency.
        };

        let mut heap_storage = Vec::new();
        let mut sorted_heap_storage = Vec::new();
        let mut heap = Vec::new();

        loop_on_all_max_heap_test_cases(|sequence| {
            assign_vec(&mut heap_storage, sequence);
            assign_vec(&mut sorted_heap_storage, sequence);

            let total_length = heap_storage.len();

            sorted_heap_storage.sort_unstable();

            for i in 0..total_length {
                assign_vec(&mut heap, &heap_storage);

                run_deletion_test(&mut heap, i, &sorted_heap_storage);
            }
        });
    }

    #[test]
    fn test_heap_delete() {
        run_heap_delete_test(heap_delete);
    }

    #[test]
    fn test_heap_delete_2() {
        run_heap_delete_test(heap_delete_2);
    }
}
