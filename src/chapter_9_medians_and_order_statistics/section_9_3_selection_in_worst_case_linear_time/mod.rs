use super::super::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice;
use super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::extra::partition_by_key;

pub mod exercises;

fn find_median_by_sorting<T: Ord + Copy>(a: &mut [T]) -> T {
    insertion_sort_slice(a);

    let middle = a.len() / 2;

    a[middle]
}

fn find_median_by_select<T: Ord + Copy>(a: &mut [T]) -> T {
    let middle = a.len() / 2;

    select_copy(a, middle)
}

fn select_copy<T: Ord + Copy>(a: &mut [T], i: usize) -> T {
    if a.len() > 1 {
        let mut group_medians = a.chunks_mut(5).map(find_median_by_sorting).collect::<Box<_>>();
        let median_of_medians = find_median_by_select(&mut group_medians);
        let (left, middle, right) = partition_by_key(a, &median_of_medians);

        if i < left.len() {
            select_copy(left, i)
        } else {
            let k = left.len() + middle.len();

            if i < k {
                middle[0]
            } else {
                select_copy(right, i - k)
            }
        }
    } else {
        a[0]
    }
}

pub fn select<T: Ord>(a: &mut [T], p: usize, r: usize, i: usize) -> &T {
    select_copy(&mut a[p..r].iter().collect::<Box<_>>(), i)
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_all_select_tests;
    use super::select;

    #[test]
    fn test_select() {
        run_all_select_tests(select);
    }
}
