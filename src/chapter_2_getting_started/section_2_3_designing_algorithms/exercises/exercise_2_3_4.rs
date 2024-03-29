pub fn insertion_sort_recursive<T: Ord + Clone>(a: &mut [T]) {
    match a.split_last_mut() {
        None => (),
        Some((key_ref, elements)) => {
            insertion_sort_recursive(elements);

            let key = key_ref.clone();
            let mut i = a.len() - 1;

            while i > 0 && a[i - 1] > key {
                a[i] = a[i - 1].clone();

                i -= 1;
            }

            a[i] = key;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_insertion_sort_recursive() {
        test_utilities::run_all_sorting_tests(super::insertion_sort_recursive);
    }
}
