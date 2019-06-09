use super::super::super::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice;
use super::super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::extra::partition_by_key;
use super::super::section_9_3_selection_in_worst_case_linear_time::select_copy;

fn find_median_by_sorting<T: Ord + Copy>(a: &mut [T]) -> T {
    insertion_sort_slice(a);

    let middle = a.len() / 2;

    a[middle]
}

fn find_median_by_select<T: Ord + Copy>(a: &mut [T]) -> T {
    let middle = a.len() / 2;

    select_copy(a, middle)
}

fn weighted_select_copy<T: Ord + Copy, F: FnMut(T) -> f64>(a: &mut [T], target: f64, f: F) -> usize {
    fn helper<T: Ord + Copy, F: FnMut(T) -> f64>(a: &mut [T], target: f64, mut f: F, offset: usize) -> usize {
        if a.is_empty() {
            offset
        } else {
            let mut group_medians = a.chunks_mut(5).map(find_median_by_sorting).collect::<Box<_>>();
            let median_of_medians = find_median_by_select(&mut group_medians);
            let (left, middle, _) = partition_by_key(a, &median_of_medians);

            // TODO: Use `Iterator::copied`: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied.

            let left_middle_sum = left.iter().map(|&x| f(x)).sum::<f64>() + f(middle[0]);
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

pub fn weighted_median<T: Ord, F: FnMut(&T) -> f64>(a: &mut [T], f: F) -> &T {
    let mut refs = a.iter().collect::<Box<_>>();
    let index = weighted_select_copy(&mut refs, 0.5, f);

    refs[index]
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
