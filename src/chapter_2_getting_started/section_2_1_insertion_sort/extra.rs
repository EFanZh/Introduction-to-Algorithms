use std::cmp::Ordering;

pub fn insertion_sort_slice_by<T, F: FnMut(&T, &T) -> Ordering>(a: &mut [T], mut compare: F) {
    for i in 1..a.len() {
        let x = &a[i];

        let p = match a[..i].binary_search_by(|y| compare(y, x)) {
            Ok(p) | Err(p) => p,
        };

        a[p..=i].rotate_right(1);
    }
}

pub fn insertion_sort_slice_by_key<T, K: Ord, F: FnMut(&T) -> K>(a: &mut [T], mut f: F) {
    insertion_sort_slice_by(a, |lhs, rhs| f(lhs).cmp(&f(rhs)));
}

pub fn insertion_sort_slice<T: Ord>(a: &mut [T]) {
    insertion_sort_slice_by(a, T::cmp);
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
    use crate::test_utilities;

    #[test]
    fn test_insertion_sort_slice() {
        test_utilities::run_all_sorting_tests(super::insertion_sort_slice);
    }

    #[test]
    fn test_insertion_sort_slice_by() {
        test_utilities::run_all_sorting_tests(|a| super::insertion_sort_slice_by(a, i32::cmp));
    }

    #[test]
    fn test_insertion_sort_slice_by_key() {
        test_utilities::run_all_sorting_tests(|a| super::insertion_sort_slice_by_key(a, |x| *x));
    }

    #[test]
    fn test_insertion_sort_tail_recursive() {
        test_utilities::run_all_sorting_tests(super::insertion_sort_tail_recursive);
    }
}
