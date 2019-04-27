use super::section_6_1_heaps::{left, right};

pub mod exercises;

// Max-Heapify(A, i)
//
//  1  l = Left(i)
//  2  r = Right(i)
//  3  if l ≤ A.heap-size and A[l] > A[i]
//  4      largest = l
//  5  else largest = i
//  6  if r ≤ A.heap-size and A[r] > A[largest]
//  7      largest = r
//  8  if largest ≠ i
//  9      exchange A[i] with A[largest]
// 10      Max-Heapify(A, largest)

pub fn max_heapify<T: Ord>(a: &mut [T], i: usize) {
    let heap_size = a.len();
    let l = left(i);
    let r = right(i);
    let mut largest = if l < heap_size && a[l] > a[i] { l } else { i };

    if r < heap_size && a[r] > a[largest] {
        largest = r
    }

    if largest != i {
        a.swap(i, largest);

        max_heapify(a, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::max_heapify;

    pub fn run_max_heapify_tests<F: Fn(&mut [i32], usize)>(f: F) {
        let run_single_test = |a: &mut [i32], i, b: &[i32]| {
            f(a.as_mut(), i);

            assert_eq!(a, b);
        };

        run_single_test(
            &mut [16, 4, 10, 14, 7, 9, 3, 2, 8, 1],
            1,
            &[16, 14, 10, 8, 7, 9, 3, 2, 4, 1],
        );

        run_single_test(
            &mut [27, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0],
            2,
            &[27, 17, 10, 16, 13, 9, 1, 5, 7, 12, 4, 8, 3, 0],
        )
    }

    #[test]
    fn test_max_heapify() {
        run_max_heapify_tests(max_heapify);
    }
}
