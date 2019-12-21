use super::super::super::super::chapter_7_quicksort::section_7_3_a_randomized_version_of_quicksort::randomized_partition;
use std::cmp::Ordering;

pub fn iterative_randomized_select<T: Ord>(a: &mut [T], mut p: usize, mut r: usize, mut i: usize) -> &mut T {
    loop {
        if r - p == 1 {
            return &mut a[p];
        }

        let q = randomized_partition(a, p, r);
        let k = q - p;

        match i.cmp(&k) {
            Ordering::Less => r = q,
            Ordering::Equal => return &mut a[q],
            Ordering::Greater => {
                p = q + 1;
                i -= k + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::tests::run_all_select_tests;
    use super::iterative_randomized_select;

    #[test]
    fn test_iterative_randomized_select() {
        run_all_select_tests(iterative_randomized_select);
    }
}
