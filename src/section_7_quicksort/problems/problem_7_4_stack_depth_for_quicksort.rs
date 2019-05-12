use super::super::section_7_1_description_of_quicksort::partition;

// Tail-Recursive-Quicksort(A, p, r)
//
// while p < r
//     // Partition and sort left subarray.
//     q = Partition(A, p, r)
//     Tail-Recursive-Quicksort(A, p, q - 1)
//     p = q + 1

pub fn tail_recursive_quicksort<T: Ord>(a: &mut [T], mut p: usize, r: usize) {
    while r - p > 1 {
        // Partition and sort left subarray.

        let q = partition(a, p, r);

        tail_recursive_quicksort(a, p, q);

        p = q + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::tail_recursive_quicksort;

    #[test]
    fn test_tail_recursive_quicksort() {
        run_all_sorting_tests(|a| tail_recursive_quicksort(a, 0, a.len()));
    }
}
