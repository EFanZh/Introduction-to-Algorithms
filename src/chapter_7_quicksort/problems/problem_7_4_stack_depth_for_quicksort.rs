use super::super::section_7_1_description_of_quicksort;

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

        let q = section_7_1_description_of_quicksort::partition(a, p, r);

        tail_recursive_quicksort(a, p, q);

        p = q + 1;
    }
}

pub fn tail_recursive_quicksort_limited_stack_depth<T: Ord>(a: &mut [T], mut p: usize, mut r: usize) {
    while r - p > 1 {
        // Partition

        let q = section_7_1_description_of_quicksort::partition(a, p, r);

        if q - p < r - (p + 1) {
            // Sort left subarray.

            tail_recursive_quicksort(a, p, q);

            p = q + 1;
        } else {
            // Sort right subarray.

            tail_recursive_quicksort(a, p + 1, r);

            r = q;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_tail_recursive_quicksort() {
        test_utilities::run_all_sorting_tests(|a| super::tail_recursive_quicksort(a, 0, a.len()));
    }

    #[test]
    fn test_tail_recursive_quicksort_limited_stack_depth() {
        test_utilities::run_all_sorting_tests(|a| super::tail_recursive_quicksort_limited_stack_depth(a, 0, a.len()));
    }
}
