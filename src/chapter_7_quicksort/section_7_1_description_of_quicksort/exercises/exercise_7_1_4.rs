use super::super::partition;

pub fn reverse_quicksort<T: Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let mut q = partition(a, p, r);

        a[p..r].reverse();
        q = p + r - 1 - q;

        reverse_quicksort(a, p, q);
        reverse_quicksort(a, q + 1, r);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_reverse_quicksort() {
        test_utilities::run_all_reverse_sorting_tests(|a| super::reverse_quicksort(a, 0, a.len()));
    }
}
