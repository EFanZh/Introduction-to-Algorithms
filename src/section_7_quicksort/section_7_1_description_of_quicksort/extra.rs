pub fn partition_slice<T: Ord>(a: &mut [T]) -> usize {
    let (x, s) = a.split_last_mut().unwrap();

    let mut i = 0;

    for j in 0..s.len() {
        if s[j] < *x {
            s.swap(i, j);

            i += 1;
        }
    }

    a.swap(i, a.len() - 1);

    i
}

pub fn quicksort_slice<T: Ord>(a: &mut [T]) {
    if !a.is_empty() {
        let q = partition_slice(a);

        quicksort_slice(&mut a[..q]);
        quicksort_slice(&mut a[q + 1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::quicksort_slice;

    #[test]
    fn test_quicksort_slice() {
        run_all_sorting_tests(quicksort_slice);
    }
}
