use super::super::super::section_6_1_heaps::parent;
use super::super::super::section_6_2_maintaining_the_heap_property::exercises::exercise_6_2_2::min_heapify;

// Heap-Minimum(A)
//
// 1  return A[1]

pub fn heap_minimum<T: Ord>(a: &[T]) -> &T {
    &a[0]
}

// Heap-Extract-Min(A)
//
// 1  if A.heap-size < 1
// 2      error “heap underflow”
// 3  min = A[1]
// 4  A[1] = A[A.heap-size]
// 5  A.heap-size = A.heap-size - 1
// 6  Min-Heapify(A, 1)
// 7  return min

pub fn heap_extract_min<T: Ord>(a: &mut Vec<T>) -> T {
    // Bound checking is done by `swap_remove`.

    let min = a.swap_remove(0);

    min_heapify(a, 0);

    min
}

// Heap-Decrease-Key(A, i, key)
//
// 1  if key > A[i]
// 2      error “new key is larger than current key”
// 3  A[i] = key
// 4  while i > 1 and A[Parent(i)] > A[i]
// 5      exchange A[i] with A[Parent(i)]
// 6      i = Parent(i)

pub fn heap_decrease_key<T: Ord>(a: &mut [T], mut i: usize, key: T) {
    if key > a[i] {
        panic!("new key is larger than current key");
    }

    a[i] = key;

    while i > 0 && a[parent(i)] > a[i] {
        a.swap(i, parent(i));
        i = parent(i);
    }
}

// Min-Heap-Insert(A, key)
//
// 1  A.heap-size = A.heap-size + 1
// 2  A[A.heap-size] = +∞
// 3  Heap-Decrease-Key(A, A.heap-size, key)

pub fn min_heap_insert<T: Ord>(a: &mut Vec<T>, key: T) {
    a.push(key);

    // Unable to create a +∞ value, inline the call to `heap_decrease_key` manually.

    let mut i = a.len() - 1;

    while i > 0 && a[parent(i)] > a[i] {
        a.swap(i, parent(i));
        i = parent(i);
    }
}

#[cfg(test)]
mod tests {
    use super::{heap_decrease_key, heap_extract_min, heap_minimum, min_heap_insert};

    #[test]
    fn test_heap_minimum() {
        assert_eq!(heap_minimum(&[1]), &1);
        assert_eq!(heap_minimum(&[1, 2, 3]), &1);
    }

    #[test]
    fn test_heap_extract_min() {
        fn run_single_test(mut a: Vec<i32>, expected_value: i32, expected_heap: &[i32]) {
            assert_eq!(heap_extract_min(&mut a), expected_value);
            assert_eq!(a.as_slice(), expected_heap);
        }

        run_single_test(vec![0], 0, &[]);
        run_single_test(vec![0, 1], 0, &[1]);
        run_single_test(vec![0, 1, 2], 0, &[1, 2]);
        run_single_test(vec![0, 1, 2, 3], 0, &[1, 3, 2]);

        run_single_test(
            vec![0, 2, 6, 10, 3, 7, 8, 11, 15, 9, 13, 14],
            0,
            &[2, 3, 6, 10, 9, 7, 8, 11, 15, 14, 13],
        );
    }

    #[test]
    fn test_heap_decrease_key() {
        fn run_single_test(mut a: Vec<i32>, i: usize, key: i32, expected: &[i32]) {
            heap_decrease_key(&mut a, i, key);

            assert_eq!(a.as_slice(), expected);
        }

        run_single_test(vec![7], 0, 0, &[0]);

        run_single_test(vec![7, 8], 0, 1, &[1, 8]);
        run_single_test(vec![3, 8], 1, 5, &[3, 5]);
        run_single_test(vec![3, 8], 1, 0, &[0, 3]);

        run_single_test(vec![2, 8, 13], 0, 1, &[1, 8, 13]);
        run_single_test(vec![2, 8, 13], 1, 7, &[2, 7, 13]);
        run_single_test(vec![2, 8, 13], 1, 0, &[0, 2, 13]);
        run_single_test(vec![2, 8, 13], 2, 11, &[2, 8, 11]);
        run_single_test(vec![2, 8, 13], 2, 7, &[2, 8, 7]);
        run_single_test(vec![2, 8, 13], 2, 1, &[1, 8, 2]);

        run_single_test(
            vec![1, 3, 7, 9, 10, 8, 14, 15, 13, 16],
            8,
            2,
            &[1, 2, 7, 3, 10, 8, 14, 15, 9, 16],
        );
    }

    #[test]
    fn test_min_heap_insert() {
        fn run_single_test(mut a: Vec<i32>, key: i32, expected: &[i32]) {
            min_heap_insert(&mut a, key);

            assert_eq!(a.as_slice(), expected);
        }

        run_single_test(vec![], 0, &[0]);
        run_single_test(vec![0], -1, &[-1, 0]);
        run_single_test(vec![0], 0, &[0, 0]);
        run_single_test(vec![0], 1, &[0, 1]);
        run_single_test(vec![0, 3], -1, &[-1, 3, 0]);
        run_single_test(vec![0, 3], 0, &[0, 3, 0]);
        run_single_test(vec![0, 3], 1, &[0, 3, 1]);
        run_single_test(vec![0, 3], 3, &[0, 3, 3]);
        run_single_test(vec![0, 3], 4, &[0, 3, 4]);

        run_single_test(
            vec![0, 2, 6, 10, 3, 7, 8, 11, 15, 9, 13, 14],
            5,
            &[0, 2, 5, 10, 3, 6, 8, 11, 15, 9, 13, 14, 7],
        );
    }
}
