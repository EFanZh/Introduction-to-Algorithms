use super::super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::extra::partition_by_key;
use super::super::section_9_3_selection_in_worst_case_linear_time::select;

fn weighted_select<T: Ord + Clone, F: FnMut(&T) -> f64>(a: &mut [T], target: f64, f: F) -> usize {
    fn helper<T: Ord + Clone, F: FnMut(&T) -> f64>(a: &mut [T], target: f64, mut f: F, offset: usize) -> usize {
        if a.is_empty() {
            offset
        } else {
            let median = select(a, 0, a.len(), a.len() / 2).clone();
            let (left, middle, _) = partition_by_key(a, &median);

            let left_middle_sum = left.iter().map(&mut f).sum::<f64>() + f(&middle[0]);
            let left_len = left.len();

            if left_middle_sum < target {
                helper(
                    &mut a[left_len + 1..],
                    target - left_middle_sum,
                    f,
                    offset + left_len + 1,
                )
            } else {
                helper(&mut a[..left_len], target, f, offset)
            }
        }
    }

    helper(a, target, f, 0)
}

pub fn weighted_median<T: Ord + Clone, F: FnMut(&T) -> f64>(a: &mut [T], f: F) -> &mut T {
    let index = weighted_select(a, 0.5, f);

    &mut a[index]
}

#[cfg(test)]
mod tests {
    use super::weighted_median;

    #[test]
    fn test_weighted_median() {
        fn run_one_test<F: FnMut(i32) -> f64>(a: &mut [i32], mut f: F, expected_result: i32) {
            assert_eq!(*weighted_median(a, |&x| f(x)), expected_result);
        }

        run_one_test(&mut [7], |_| 1.0, 7);
        run_one_test(&mut [10, 35, 5, 10, 15, 5, 20], |x| f64::from(x) / 100.0, 20);
    }
}
