use super::{super::section_2_3_designing_algorithms::merge_sort, utilities::run_all_sorting_tests};
use crate::test::Bencher;

fn merge_sort_helper<T: Ord + Clone>(a: &mut [T]) {
    merge_sort(a, 0, a.len());
}

#[bench]
fn test_merge_sort(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(merge_sort_helper));
}

mod exercises;
mod problems;
