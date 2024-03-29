use super::super::super::section_6_1_heaps;
use super::super::super::section_6_2_maintaining_the_heap_property;

pub fn heap_delete<T: Ord>(a: &mut Vec<T>, mut i: usize) {
    a.swap_remove(i);

    if i < a.len() {
        if i == 0 || a[i] <= a[section_6_1_heaps::parent(i)] {
            // The heap property holds on a[i]’s parent, we make sure it holds on its children.

            section_6_2_maintaining_the_heap_property::max_heapify(a, i);
        } else {
            // The heap property doesn’t hold on a[i]’s parent, we fix it by swapping it with its parent.

            loop {
                a.swap(i, section_6_1_heaps::parent(i));

                i = section_6_1_heaps::parent(i);

                if i == 0 || a[i] <= a[section_6_1_heaps::parent(i)] {
                    break;
                }
            }
        }
    }
}

pub fn heap_delete_2<T: Ord>(a: &mut Vec<T>, mut i: usize) {
    if a[a.len() - 1] <= a[i] {
        a.swap_remove(i);

        section_6_2_maintaining_the_heap_property::max_heapify(a, i);
    } else {
        a.swap_remove(i);

        while i > 0 && a[i] > a[section_6_1_heaps::parent(i)] {
            a.swap(i, section_6_1_heaps::parent(i));

            i = section_6_1_heaps::parent(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    fn run_heap_delete_test<F: FnMut(&mut Vec<i32>, usize)>(mut f: F) {
        let mut run_deletion_test = |heap: &mut Vec<i32>, i, expected_values: &[i32]| {
            let value = heap[i]; // Save the value to be deleted.

            f(heap, i); // Delete the value.

            assert!(test_utilities::is_max_heap(heap)); // Check max heap property.

            heap.push(value); // Put the deleted value back.

            heap.sort_unstable(); // Sort the heap.

            assert_eq!(heap.as_slice(), expected_values); // Check value consistency.
        };

        let mut heap_storage = Vec::new();
        let mut sorted_heap_storage = Vec::new();
        let mut heap = Vec::new();

        test_utilities::loop_on_all_max_heap_test_cases(|sequence| {
            test_utilities::assign_vec(&mut heap_storage, sequence);
            test_utilities::assign_vec(&mut sorted_heap_storage, sequence);

            let total_length = heap_storage.len();

            sorted_heap_storage.sort_unstable();

            for i in 0..total_length {
                test_utilities::assign_vec(&mut heap, &heap_storage);

                run_deletion_test(&mut heap, i, &sorted_heap_storage);
            }
        });
    }

    #[test]
    fn test_heap_delete() {
        run_heap_delete_test(super::heap_delete);
    }

    #[test]
    fn test_heap_delete_2() {
        run_heap_delete_test(super::heap_delete_2);
    }
}
