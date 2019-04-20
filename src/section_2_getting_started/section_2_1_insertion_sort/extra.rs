pub fn insertion_sort_tail_recursive<T: Ord + Clone>(a: &mut [T]) {
    fn insert_key<T: Ord + Clone>(a: &mut [T], key: T) {
        let free_slot_index = a.len() - 1;

        if free_slot_index > 0 && a[free_slot_index - 1] > key {
            a[free_slot_index] = a[free_slot_index - 1].clone();

            insert_key(&mut a[..free_slot_index], key);
        } else {
            a[free_slot_index] = key;
        }
    }

    fn insertion_sort_helper<T: Ord + Clone>(a: &mut [T], num_sorted: usize) {
        if num_sorted < a.len() {
            let key = a[num_sorted].clone();
            let next_separator = num_sorted + 1;

            insert_key(&mut a[0..next_separator], key);

            insertion_sort_helper(a, next_separator);
        }
    }

    insertion_sort_helper(a, 0);
}

#[cfg(test)]
mod tests {
    use super::insertion_sort_tail_recursive;
    use crate::test_utilities::run_all_sorting_tests;

    #[test]
    fn test_insertion_sort_tail_recursive() {
        run_all_sorting_tests(insertion_sort_tail_recursive);
    }
}
