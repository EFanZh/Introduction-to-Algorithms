use super::section_6_1_heaps::parent;
use super::section_6_2_maintaining_the_heap_property::max_heapify;

pub mod exercises;
pub mod extra;
pub mod problems;

// Heap-Maximum(A)
//
// 1  return A[1]

pub fn heap_maximum<T: Ord>(a: &[T]) -> &T {
    &a[0]
}

// Heap-Extract-Max(A)
//
// 1  if A.heap-size < 1
// 2      error “heap underflow”
// 3  max = A[1]
// 4  A[1] = A[A.heap-size]
// 5  A.heap-size = A.heap-size - 1
// 6  Max-Heapify(A, 1)
// 7  return max

pub fn heap_extract_max<T: Ord>(a: &mut Vec<T>) -> T {
    // Bound checking is done by `swap_remove`.

    let max = a.swap_remove(0);

    max_heapify(a, 0);

    max
}

// Heap-Increase-Key(A, i, key)
//
// 1  if key < A[i]
// 2      error “new key is smaller than current key”
// 3  A[i] = key
// 4  while i > 1 and A[Parent(i)] < A[i]
// 5      exchange A[i] with A[Parent(i)]
// 6      i = Parent(i)

pub fn heap_increase_key<T: Ord>(a: &mut [T], mut i: usize, key: T) {
    if key < a[i] {
        panic!("new key is smaller than current key");
    }

    a[i] = key;

    while i > 0 && a[parent(i)] < a[i] {
        a.swap(i, parent(i));
        i = parent(i);
    }
}

// Max-Heap-Insert(A, key)
//
// 1  A.heap-size = A.heap-size + 1
// 2  A[A.heap-size] = -∞
// 3  Heap-Increase-Key(A, A.heap-size, key)

pub fn max_heap_insert<T: Ord>(a: &mut Vec<T>, key: T) {
    a.push(key);

    // Unable to create a -∞ value, inline the call to `heap_increase_key` manually.

    let mut i = a.len() - 1;

    while i > 0 && a[parent(i)] < a[i] {
        a.swap(i, parent(i));
        i = parent(i);
    }
}

#[cfg(test)]
mod tests {
    use super::{heap_extract_max, heap_increase_key, heap_maximum, max_heap_insert};

    #[test]
    fn test_heap_maximum() {
        assert_eq!(heap_maximum(&[1]), &1);
        assert_eq!(heap_maximum(&[3, 2, 1]), &3);
    }

    #[test]
    fn test_heap_extract_max() {
        fn run_single_test(mut a: Vec<i32>, expected_value: i32, expected_heap: &[i32]) {
            assert_eq!(heap_extract_max(&mut a), expected_value);
            assert_eq!(a.as_slice(), expected_heap);
        }

        run_single_test(vec![0], 0, &[]);
        run_single_test(vec![1, 0], 1, &[0]);
        run_single_test(vec![2, 1, 0], 2, &[1, 0]);
        run_single_test(vec![3, 2, 1, 0], 3, &[2, 0, 1]);

        run_single_test(
            vec![15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1],
            15,
            &[13, 12, 9, 5, 6, 8, 7, 4, 0, 1, 2],
        );
    }

    pub fn run_heap_increase_key_test<F: Fn(&mut [i32], usize, i32)>(f: F) {
        let run_single_test = |mut a: Vec<i32>, i: usize, key: i32, expected: &[i32]| {
            f(&mut a, i, key);

            assert_eq!(a.as_slice(), expected);
        };

        run_single_test(vec![0], 0, 7, &[7]);

        run_single_test(vec![1, 0], 0, 7, &[7, 0]);
        run_single_test(vec![5, 0], 1, 3, &[5, 3]);
        run_single_test(vec![5, 0], 1, 8, &[8, 5]);

        run_single_test(vec![13, 7, 2], 0, 14, &[14, 7, 2]);
        run_single_test(vec![13, 7, 2], 1, 8, &[13, 8, 2]);
        run_single_test(vec![13, 7, 2], 1, 15, &[15, 13, 2]);
        run_single_test(vec![13, 7, 2], 2, 4, &[13, 7, 4]);
        run_single_test(vec![13, 7, 2], 2, 8, &[13, 7, 8]);
        run_single_test(vec![13, 7, 2], 2, 14, &[14, 7, 13]);

        run_single_test(
            vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1],
            8,
            15,
            &[16, 15, 10, 14, 7, 9, 3, 2, 8, 1],
        );
    }

    #[test]
    fn test_heap_increase_key() {
        run_heap_increase_key_test(heap_increase_key);
    }

    #[test]
    fn test_max_heap_insert() {
        fn run_single_test(mut a: Vec<i32>, key: i32, expected: &[i32]) {
            max_heap_insert(&mut a, key);

            assert_eq!(a.as_slice(), expected);
        }

        run_single_test(vec![], 0, &[0]);
        run_single_test(vec![0], -1, &[0, -1]);
        run_single_test(vec![0], 0, &[0, 0]);
        run_single_test(vec![0], 1, &[1, 0]);
        run_single_test(vec![3, 0], -1, &[3, 0, -1]);
        run_single_test(vec![3, 0], 0, &[3, 0, 0]);
        run_single_test(vec![3, 0], 1, &[3, 0, 1]);
        run_single_test(vec![3, 0], 3, &[3, 0, 3]);
        run_single_test(vec![3, 0], 4, &[4, 0, 3]);

        run_single_test(
            vec![15, 13, 9, 5, 12, 8, 7, 4, 0, 6, 2, 1],
            10,
            &[15, 13, 10, 5, 12, 9, 7, 4, 0, 6, 2, 1, 8],
        )
    }
}
