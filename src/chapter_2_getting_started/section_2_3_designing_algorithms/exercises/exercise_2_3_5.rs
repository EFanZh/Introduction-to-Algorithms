use std::mem;

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

    (left < a.len() && a[left] == *v).then(|| left)
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

    (base < a.len() && a[base] == *v).then(|| base)
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
        base += 1;
    }

    (base < a.len() && *v == a[base]).then(|| base)
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
        } else if left < a.len() && a[left] == *v {
            Some(left)
        } else {
            None
        }
    }

    binary_search_recursive_helper(a, v, 0, a.len())
}

pub fn binary_search_recursive_non_tail<T: Ord>(a: &[T], v: &T) -> Option<usize> {
    fn lower_bound_non_tail<T: Ord>(a: &[T], v: &T) -> usize {
        if a.is_empty() {
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

    (index < a.len() && a[index] == *v).then(|| index)
}

pub fn binary_search_recursive_pointer<T: Ord>(a: &[T], v: &T) -> Option<usize> {
    fn lower_bound_pointer<T: Ord>(a: &[T], v: &T) -> *const T {
        if a.is_empty() {
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

    let index = (lower_bound_pointer(a, v) as usize - a.as_ptr() as usize) / mem::size_of::<T>();

    (index < a.len() && a[index] == *v).then(|| index)
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
        } else if base < a.len() && a[base] == *v {
            Some(base)
        } else {
            None
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

            (index < a.len() && a[index] == *v).then(|| index)
        }
    }

    let size = a.len();

    if size == 0 {
        None
    } else {
        binary_search_recursive_rust_helper(a, v, 0, size)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        binary_search_iterative, binary_search_iterative_libcxx, binary_search_iterative_rust, binary_search_recursive,
        binary_search_recursive_libcxx, binary_search_recursive_non_tail, binary_search_recursive_pointer,
        binary_search_recursive_rust,
    };
    use crate::test_utilities;

    #[test]
    fn test_binary_search_iterative() {
        test_utilities::run_all_binary_search_tests(binary_search_iterative);
    }

    #[test]
    fn test_binary_search_iterative_libcxx() {
        test_utilities::run_all_binary_search_tests(binary_search_iterative_libcxx);
    }

    #[test]
    fn test_binary_search_iterative_rust() {
        test_utilities::run_all_binary_search_tests(binary_search_iterative_rust);
    }

    #[test]
    fn test_binary_search_recursive() {
        test_utilities::run_all_binary_search_tests(binary_search_recursive);
    }

    #[test]
    fn test_binary_search_recursive_non_tail() {
        test_utilities::run_all_binary_search_tests(binary_search_recursive_non_tail);
    }

    #[test]
    fn test_binary_search_recursive_pointer() {
        test_utilities::run_all_binary_search_tests(binary_search_recursive_pointer);
    }

    #[test]
    fn test_binary_search_recursive_libcxx() {
        test_utilities::run_all_binary_search_tests(binary_search_recursive_libcxx);
    }

    #[test]
    fn test_binary_search_recursive_rust() {
        test_utilities::run_all_binary_search_tests(binary_search_recursive_rust);
    }
}
