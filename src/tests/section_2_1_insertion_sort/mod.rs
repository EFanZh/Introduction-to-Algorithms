use super::{super::section_2_1_insertion_sort::insertion_sort, utilities::run_all_sorting_tests};
use crate::test::Bencher;

#[bench]
fn test_insertion_sort(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(insertion_sort));
}

mod exercises;
mod extra;
