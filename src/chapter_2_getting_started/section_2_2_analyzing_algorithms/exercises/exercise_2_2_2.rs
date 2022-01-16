pub fn selection_sort<T: Ord + Clone>(a: &mut [T]) {
    let n = a.len();

    for i in (1..n).map(|x| x - 1) {
        let mut min_index = i;

        for j in i + 1..n {
            if a[j] < a[min_index] {
                min_index = j;
            }
        }

        a.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::run_all_sorting_tests;
    use super::selection_sort;

    #[test]
    pub fn test_selection_sort() {
        run_all_sorting_tests(selection_sort);
    }
}
