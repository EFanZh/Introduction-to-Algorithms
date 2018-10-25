use super::utilities::MaxSentinel;

// Merge(A, p, q, r)
//  1  n1 = q - p + 1
//  2  n2 = r - q
//  3  let L[1‥n1 + 1] and R[1‥n2 + 1] be new arrays
//  4  for i = 1 to n1
//  5      L[i] = A[p + i - 1]
//  6  for j = 1 to n2
//  7      R[j] = A[q + j]
//  8  L[n1 + 1] = ∞
//  9  R[n2 + 1] = ∞
// 10  i = 1
// 11  j = 1
// 12  for k = p to r
// 13      if L[i] ≤ R[j]
// 14          A[k] = L[i]
// 15          i = i + 1
// 16      else A[k] = R[j]
// 17          j = j + 1

pub fn merge<T: Clone + Ord>(a: &mut [T], p: usize, q: usize, r: usize) {
    let n1 = q - p;
    let n2 = r - q;
    let (mut left, mut right) = (vec![Default::default(); n1 + 1], vec![Default::default(); n2 + 1]);

    for i in 0..n1 {
        left[i] = a[p + i].clone().into();
    }

    for j in 0..n2 {
        right[j] = a[q + j].clone().into();
    }

    left[n1] = MaxSentinel::max();
    right[n2] = MaxSentinel::max();

    let mut i = 0;
    let mut j = 0;

    for a_k in &mut a[p..r] {
        if left[i] <= right[j] {
            *a_k = left[i].take_unwrap();
            i += 1;
        } else {
            *a_k = right[j].take_unwrap();
            j += 1;
        }
    }
}

// Merge-Sort(A, p, r)
//   1 if p < r
//   2     q = ⌊(p + r) / 2⌋
//   3     Merge-Sort(A, p, q)
//   4     Merge-Sort(A, q + 1, r)
//   5     Merge(A, p, q, r)

pub fn merge_sort<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q = p + (r - p) / 2;

        merge_sort(a, p, q);
        merge_sort(a, q, r);
        merge(a, p, q, r);
    }
}

pub mod exercises {
    pub mod exercise_2_3_2 {
        pub fn merge<T: Clone + Ord>(a: &mut [T], p: usize, q: usize, r: usize) {
            let copied = a[p..r].to_vec();
            let (left, right) = copied.split_at(q - p);

            let mut i = 0;
            let mut j = 0;

            for k in p..r {
                if i < left.len() {
                    if j < right.len() {
                        if left[i] <= right[j] {
                            a[k] = left[i].clone();

                            i += 1;
                        } else {
                            a[k] = right[j].clone();

                            j += 1;
                        }
                    } else {
                        (&mut a[k..r]).clone_from_slice(&left[i..]);

                        break;
                    }
                } else {
                    (&mut a[k..r]).clone_from_slice(&right[j..]);

                    break;
                }
            }
        }
    }

    pub mod exercise_2_3_4 {
        pub fn insertion_sort_recursive<T: Ord + Clone>(a: &mut [T]) {
            match a.split_last_mut() {
                None => (),
                Some((key_ref, elements)) => {
                    insertion_sort_recursive(elements);

                    let key = key_ref.clone();
                    let mut i = a.len() - 1;

                    while i > 0 && a[i - 1] > key {
                        a[i] = a[i - 1].clone();

                        i -= 1;
                    }

                    a[i] = key;
                }
            }
        }
    }

    pub mod exercise_2_3_5 {
        pub fn binary_search_iterative<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            let mut left = 0;
            let mut right = a.len();

            while left < right {
                let middle = left + (right - left) / 2;

                if a[middle] < *v {
                    left = middle + 1;
                } else {
                    right = middle;
                }
            }

            if left < a.len() && a[left] == *v {
                Some(left)
            } else {
                None
            }
        }

        pub fn binary_search_iterative_libcxx<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            let mut base = 0;
            let mut size = a.len();

            while size > 0 {
                let half = size / 2;
                let middle = base + half;

                if a[middle] < *v {
                    base = middle + 1;
                    size -= half + 1;
                } else {
                    size = half;
                }
            }

            if base < a.len() && a[base] == *v {
                Some(base)
            } else {
                None
            }
        }

        pub fn binary_search_iterative_rust<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            let mut size = a.len();

            if size == 0 {
                return None;
            }

            let mut base = 0;

            while size > 1 {
                let half = size / 2;
                let middle = base + half;

                if a[middle] < *v {
                    base = middle;
                }

                size -= half;
            }

            if a[base] < *v {
                base += 1
            }

            if base < a.len() && *v == a[base] {
                Some(base)
            } else {
                None
            }
        }

        pub fn binary_search_recursive<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            fn binary_search_recursive_helper<T: Ord>(a: &[T], v: &T, left: usize, right: usize) -> Option<usize> {
                if left < right {
                    let middle = left + (right - left) / 2;

                    if a[middle] < *v {
                        binary_search_recursive_helper(a, v, middle + 1, right)
                    } else {
                        binary_search_recursive_helper(a, v, left, middle)
                    }
                } else {
                    if left < a.len() && a[left] == *v {
                        Some(left)
                    } else {
                        None
                    }
                }
            }

            binary_search_recursive_helper(a, v, 0, a.len())
        }

        pub fn binary_search_recursive_non_tail<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            fn lower_bound_non_tail<T: Ord>(a: &[T], v: &T) -> usize {
                if a.len() == 0 {
                    0
                } else {
                    let middle = a.len() / 2;

                    if a[middle] < *v {
                        middle + 1 + lower_bound_non_tail(&a[middle + 1..], v)
                    } else {
                        lower_bound_non_tail(&a[..middle], v)
                    }
                }
            }

            let index = lower_bound_non_tail(a, v);

            if index < a.len() && a[index] == *v {
                Some(index)
            } else {
                None
            }
        }

        pub fn binary_search_recursive_libcxx<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            fn binary_search_recursive_libcxx_helper<T: Ord>(
                a: &[T],
                v: &T,
                base: usize,
                size: usize,
            ) -> Option<usize> {
                if size > 0 {
                    let half = size / 2;
                    let middle = base + half;

                    if a[middle] < *v {
                        binary_search_recursive_libcxx_helper(a, v, middle + 1, size - half - 1)
                    } else {
                        binary_search_recursive_libcxx_helper(a, v, base, half)
                    }
                } else {
                    if base < a.len() && a[base] == *v {
                        Some(base)
                    } else {
                        None
                    }
                }
            }

            binary_search_recursive_libcxx_helper(a, v, 0, a.len())
        }

        pub fn binary_search_recursive_rust<T: Ord>(a: &[T], v: &T) -> Option<usize> {
            fn binary_search_recursive_rust_helper<T: Ord>(a: &[T], v: &T, base: usize, size: usize) -> Option<usize> {
                if size > 1 {
                    let half = size / 2;
                    let middle = base + half;

                    if a[middle] < *v {
                        binary_search_recursive_rust_helper(a, v, middle, size - half)
                    } else {
                        binary_search_recursive_rust_helper(a, v, base, size - half)
                    }
                } else {
                    let index = if a[base] < *v { base + 1 } else { base };

                    if index < a.len() && a[index] == *v {
                        Some(index)
                    } else {
                        None
                    }
                }
            }

            let size = a.len();

            if size == 0 {
                None
            } else {
                binary_search_recursive_rust_helper(a, v, 0, size)
            }
        }
    }
}
