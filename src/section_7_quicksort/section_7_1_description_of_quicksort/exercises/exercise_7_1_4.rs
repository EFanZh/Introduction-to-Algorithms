use super::super::partition;

pub fn reverse_quicksort<T: Ord>(a: &mut [T], p: usize, r: usize) {
    if p + 1 < r {
        let mut q = partition(a, p, r);

        a[p..r].reverse();
        q = p + r - 1 - q;

        reverse_quicksort(a, p, q);
        reverse_quicksort(a, q + 1, r);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::run_all_reverse_sorting_tests;
    use super::reverse_quicksort;

    #[test]
    fn test_reverse_quicksort() {
        run_all_reverse_sorting_tests(|a| reverse_quicksort(a, 0, a.len()));
    }
}
