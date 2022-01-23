use super::section_6_2_maintaining_the_heap_property;
use super::section_6_2_maintaining_the_heap_property::exercises::exercise_6_2_2;

// Build-Max-Heap(A)
//
// 1  A.heap-size = A.length
// 2  for i = ⌊A.length / 2⌋ downto 1
// 3      Max-Heapify(A, i)

pub fn build_max_heap<T: Ord>(a: &mut [T]) {
    for i in (0..a.len() / 2).rev() {
        section_6_2_maintaining_the_heap_property::max_heapify(a, i);
    }
}

pub fn build_min_heap<T: Ord>(a: &mut [T]) {
    for i in (0..a.len() / 2).rev() {
        exercise_6_2_2::min_heapify(a, i);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_build_max_heap() {
        fn run_single_test(a: &[i32]) {
            let mut a_1 = a.to_vec();
            let mut a_2 = a_1.clone();

            super::build_max_heap(&mut a_1);

            assert!(test_utilities::is_max_heap(&a_1));

            a_1.sort_unstable();
            a_2.sort_unstable();

            assert_eq!(a_1, a_2);
        }

        run_single_test(&[]);

        run_single_test(&[0]);

        run_single_test(&[0, 0]);
        run_single_test(&[0, 1]);
        run_single_test(&[1, 0]);

        run_single_test(&[0, 0, 0]);
        run_single_test(&[0, 0, 1]);
        run_single_test(&[0, 1, 0]);
        run_single_test(&[0, 1, 1]);
        run_single_test(&[0, 1, 2]);
        run_single_test(&[0, 2, 1]);
        run_single_test(&[1, 0, 0]);
        run_single_test(&[1, 0, 1]);
        run_single_test(&[1, 0, 2]);
        run_single_test(&[1, 1, 0]);
        run_single_test(&[1, 2, 0]);
        run_single_test(&[2, 0, 1]);
        run_single_test(&[2, 1, 0]);

        // Test case from CLRS.

        run_single_test(&[4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
        run_single_test(&[5, 3, 17, 10, 84, 19, 6, 22, 9]);
    }

    #[test]
    fn test_build_min_heap() {
        fn run_single_test(a: &[i32]) {
            let mut a_1 = a.to_vec();
            let mut a_2 = a_1.clone();

            super::build_min_heap(&mut a_1);

            assert!(test_utilities::is_min_heap(&a_1));

            a_1.sort_unstable();
            a_2.sort_unstable();

            assert_eq!(a_1, a_2);
        }

        run_single_test(&[]);

        run_single_test(&[0]);

        run_single_test(&[0, 0]);
        run_single_test(&[0, 1]);
        run_single_test(&[1, 0]);

        run_single_test(&[0, 0, 0]);
        run_single_test(&[0, 0, 1]);
        run_single_test(&[0, 1, 0]);
        run_single_test(&[0, 1, 1]);
        run_single_test(&[0, 1, 2]);
        run_single_test(&[0, 2, 1]);
        run_single_test(&[1, 0, 0]);
        run_single_test(&[1, 0, 1]);
        run_single_test(&[1, 0, 2]);
        run_single_test(&[1, 1, 0]);
        run_single_test(&[1, 2, 0]);
        run_single_test(&[2, 0, 1]);
        run_single_test(&[2, 1, 0]);

        // Test case from CLRS.

        run_single_test(&[4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
        run_single_test(&[5, 3, 17, 10, 84, 19, 6, 22, 9]);
    }
}
