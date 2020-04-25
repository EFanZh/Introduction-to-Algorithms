use num_traits::Zero;

pub fn find_maximum_subarray_linear_time<T: Clone + Ord + Zero>(a: &[T]) -> (usize, usize, T) {
    let mut max_left = 0;
    let mut max_right = 0;
    let mut max_sum = a[0].clone();
    let mut max_sum_end_left = 0;
    let mut max_sum_end = a[0].clone();

    for (i, a_i) in a.iter().enumerate().skip(1) {
        if max_sum_end < T::zero() {
            max_sum_end_left = i;
            max_sum_end = a_i.clone();
        } else {
            max_sum_end = max_sum_end + a_i.clone();
        };

        if max_sum_end > max_sum {
            max_left = max_sum_end_left;
            max_right = i;
            max_sum = max_sum_end.clone();
        }
    }

    (max_left, max_right + 1, max_sum)
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_find_maximum_subarray_test_cases;
    use super::find_maximum_subarray_linear_time;

    #[test]
    fn test_find_maximum_subarray_linear_time() {
        run_find_maximum_subarray_test_cases(find_maximum_subarray_linear_time);
    }
}
