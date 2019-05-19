pub fn insertion_sort_slice<T: Ord>(a: &mut [T]) {
    for i in 1..a.len() {
        let x = &a[i];

        let p = match a[..i].binary_search(x) {
            Ok(p) => p,
            Err(p) => p,
        };

        a[p..=i].rotate_right(1);
    }
}

pub fn insertion_sort_slice_by_key<T, K: Ord, F: FnMut(&T) -> K>(a: &mut [T], mut f: F) {
    for i in 1..a.len() {
        let x = &a[i];

        let p = match a[..i].binary_search_by_key(&f(&x), &mut f) {
            Ok(p) => p,
            Err(p) => p,
        };

        a[p..=i].rotate_right(1);
    }
}

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
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::{insertion_sort_slice, insertion_sort_slice_by_key, insertion_sort_tail_recursive};

    #[test]
    fn test_insertion_sort_slice() {
        run_all_sorting_tests(insertion_sort_slice);
    }

    #[test]
    fn test_insertion_sort_slice_by_key() {
        run_all_sorting_tests(|a| insertion_sort_slice_by_key(a, |x| *x));
    }

    #[test]
    fn test_insertion_sort_tail_recursive() {
        run_all_sorting_tests(insertion_sort_tail_recursive);
    }
}
