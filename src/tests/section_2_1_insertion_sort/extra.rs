use super::{
    super::super::section_2_1_insertion_sort::extra::insertion_sort_tail_recursive, run_all_sorting_tests, Bencher,
};

#[bench]
fn test_insertion_sort_tail_recursive(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(insertion_sort_tail_recursive));
}
