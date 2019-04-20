pub fn selection_sort<T: Ord + Clone>(a: &mut [T]) {
    let n = a.len();

    if n > 1 {
        for i in 0..n - 1 {
            let mut min_index = i;

            for j in i..n {
                if a[j] < a[min_index] {
                    min_index = j;
                }
            }

            a.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;
    use crate::test_utilities::run_all_sorting_tests;

    #[test]
    pub fn test_selection_sort() {
        run_all_sorting_tests(selection_sort);
    }
}
