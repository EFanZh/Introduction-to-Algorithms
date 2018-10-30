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
    use std::mem::size_of;

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

    pub fn binary_search_recursive_pointer<T: Ord>(a: &[T], v: &T) -> Option<usize> {
        fn lower_bound_pointer<T: Ord>(a: &[T], v: &T) -> *const T {
            if a.len() == 0 {
                a.as_ptr()
            } else {
                let middle = a.len() / 2;

                if a[middle] < *v {
                    lower_bound_pointer(&a[middle + 1..], v)
                } else {
                    lower_bound_pointer(&a[..middle], v)
                }
            }
        }

        let index = (lower_bound_pointer(a, v) as usize - a.as_ptr() as usize) / size_of::<T>();

        if index < a.len() && a[index] == *v {
            Some(index)
        } else {
            None
        }
    }

    pub fn binary_search_recursive_libcxx<T: Ord>(a: &[T], v: &T) -> Option<usize> {
        fn binary_search_recursive_libcxx_helper<T: Ord>(a: &[T], v: &T, base: usize, size: usize) -> Option<usize> {
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

pub mod exercise_2_3_7 {
    pub fn two_sum(s: &[i32], x: i32) -> bool {
        let a_sorted = {
            let mut a_temp = s.to_vec();
            a_temp.sort_unstable();
            a_temp
        };

        let mut range = a_sorted.as_slice();

        while range.len() > 1 {
            let first = range[0];
            let last = range[range.len() - 1];

            if first + last < x {
                range = &range[1..];
            } else if first + last > x {
                range = &range[..range.len() - 1];
            } else {
                return true;
            }
        }

        false
    }
}
