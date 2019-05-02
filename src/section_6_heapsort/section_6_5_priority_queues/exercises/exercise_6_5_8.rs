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

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::{assign_vec, is_max_heap, loop_on_all_max_heap_test_cases};

    #[test]
    fn test_heap_delete() {
        let mut heap_storage = Vec::new();
        let mut sorted_heap_storage = Vec::new();
        let mut heap = Vec::new();

        loop_on_all_max_heap_test_cases(|sequence| {
            assign_vec(&mut heap_storage, sequence);
            assign_vec(&mut sorted_heap_storage, sequence);

            sorted_heap_storage.sort_unstable();

            for i in 0..heap_storage.len() {
                assign_vec(&mut heap, &heap_storage);

                let value = heap[i]; // Save the value to be deleted.

                super::heap_delete(&mut heap, i); // Delete the value.

                assert!(is_max_heap(&heap)); // Check max heap property.

                heap.push(value); // Put the deleted value back.

                heap.sort(); // Sort the heap.

                assert_eq!(heap, sorted_heap_storage); // Check value consistency.
            }
        });
    }
}
