use super::super::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice;
use super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::extra::partition_by_key;

pub mod exercises;

fn find_median_by_sorting<T: Ord>(a: &mut [T]) -> &mut T {
    insertion_sort_slice(a);

    let middle = a.len() / 2;

    &mut a[middle]
}

fn find_median_by_select<T: Ord + Clone>(a: &mut [T]) -> &mut T {
    let middle = a.len() / 2;

    select_slice(a, middle)
}

fn select_slice<T: Ord + Clone>(a: &mut [T], i: usize) -> &mut T {
    if a.len() > 1 {
        let mut group_medians = a
            .chunks_mut(5)
            .map(find_median_by_sorting)
            .map(|x| &*x)
            .cloned()
            .collect::<Box<_>>();

        let median_of_medians = find_median_by_select(&mut group_medians);
        let (left, middle, right) = partition_by_key(a, &median_of_medians);

        if i < left.len() {
            select_slice(left, i)
        } else {
            let k = left.len() + middle.len();

            if i < k {
                &mut middle[0]
            } else {
                select_slice(right, i - k)
            }
        }
    } else {
        &mut a[0]
    }
}

pub fn select<T: Ord + Clone>(a: &mut [T], p: usize, r: usize, i: usize) -> &mut T {
    select_slice(&mut a[p..r], i)
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
