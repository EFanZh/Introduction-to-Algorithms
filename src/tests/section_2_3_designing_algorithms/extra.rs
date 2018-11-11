use super::{
    super::{
        super::section_2_3_designing_algorithms::extra::{merge_sort_allocate_once, merge_sort_allocate_once_2},
        utilities::run_all_sorting_tests,
    },
    Bencher,
};

#[bench]
fn test_merge_sort_allocate_once(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(merge_sort_allocate_once));
}

#[bench]
fn test_merge_sort_allocate_once_2(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(merge_sort_allocate_once_2));
}
