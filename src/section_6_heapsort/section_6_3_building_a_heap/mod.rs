use super::section_6_2_maintaining_the_heap_property::max_heapify;

// Build-Max-Heap(A)
//
// 1  A.heap-size = A.length
// 2  for i = ⌊A.length / 2⌋ downto 1
// 3      Max-Heapify(A, i)

pub fn build_max_heap<T: Ord>(a: &mut [T]) {
    for i in (0..a.len() / 2).rev() {
        max_heapify(a, i);
    }
}

#[cfg(test)]
mod tests {
    use super::super::section_6_1_heaps::parent;
    use super::build_max_heap;

    fn is_max_heap<T: Ord>(a: &[T]) -> bool {
        a.iter().enumerate().skip(1).all(|(i, v)| v <= &a[parent(i)])
    }

    #[test]
    fn test_build_max_heap() {
        fn run_single_test(a: &[i32]) {
            let mut a_1 = a.to_vec();
            let mut a_2 = a_1.clone();

            build_max_heap(&mut a_1);

            assert!(is_max_heap(&a_1));

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
    }
}
