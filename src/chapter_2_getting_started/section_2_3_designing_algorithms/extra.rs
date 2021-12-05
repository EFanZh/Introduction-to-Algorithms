pub fn merge_sort_allocate_once<T: Clone + Ord>(a: &mut [T]) {
    fn merge<T: Clone + Ord>(left: &[T], right: &[T], buffer: &mut [T]) {
        let mut i = 0;
        let mut j = 0;

        for a_k in &mut *buffer {
            if left[i] <= right[j] {
                *a_k = left[i].clone();

                i += 1;

                if i == left.len() {
                    break;
                }
            } else {
                *a_k = right[j].clone();

                j += 1;

                if j == right.len() {
                    break;
                }
            }
        }

        if i == left.len() {
            buffer[i + j..].clone_from_slice(&right[j..]);
        } else {
            buffer[i + j..].clone_from_slice(&left[i..]);
        }
    }

    fn merge_sort_helper<T: Clone + Ord>(a: &mut [T], buffer: &mut [T]) {
        if a.len() > 1 {
            let (left, right) = a.split_at_mut(a.len() / 2);

            merge_sort_helper(left, &mut buffer[..left.len()]);
            merge_sort_helper(right, &mut buffer[..right.len()]);
            merge(left, right, buffer);

            a.clone_from_slice(buffer);
        }
    }

    merge_sort_helper(a, &mut a.to_vec());
}

pub fn merge_sort_allocate_once_2<T: Clone + Ord>(a: &mut [T]) {
    fn merge<T: Clone + Ord>(left: &[T], right: &[T], buffer: &mut [T]) {
        let mut i = 0;
        let mut j = 0;

        for a_k in &mut *buffer {
            if left[i] <= right[j] {
                *a_k = left[i].clone();

                i += 1;

                if i == left.len() {
                    break;
                }
            } else {
                *a_k = right[j].clone();

                j += 1;

                if j == right.len() {
                    break;
                }
            }
        }

        if i == left.len() {
            buffer[i + j..].clone_from_slice(&right[j..]);
        } else {
            buffer[i + j..].clone_from_slice(&left[i..]);
        }
    }

    fn merge_sort_to_buffer<T: Clone + Ord>(a: &mut [T], buffer: &mut [T]) {
        if a.len() > 1 {
            let middle = a.len() / 2;
            let (left, right) = a.split_at_mut(middle);
            let (left_buffer, right_buffer) = buffer.split_at_mut(middle);

            merge_sort_to_self(left, left_buffer);
            merge_sort_to_self(right, right_buffer);
            merge(left, right, buffer);
        }
    }

    fn merge_sort_to_self<T: Clone + Ord>(a: &mut [T], buffer: &mut [T]) {
        if a.len() > 1 {
            let middle = a.len() / 2;
            let (left, right) = a.split_at_mut(middle);
            let (left_buffer, right_buffer) = buffer.split_at_mut(middle);

            merge_sort_to_buffer(left, left_buffer);
            merge_sort_to_buffer(right, right_buffer);
            merge(left_buffer, right_buffer, a);
        }
    }

    merge_sort_to_self(a, &mut a.to_vec());
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::{merge_sort_allocate_once, merge_sort_allocate_once_2};

    #[test]
    fn test_merge_sort_allocate_once() {
        run_all_sorting_tests(merge_sort_allocate_once);
    }

    #[test]
    fn test_merge_sort_allocate_once_2() {
        run_all_sorting_tests(merge_sort_allocate_once_2);
    }
}
