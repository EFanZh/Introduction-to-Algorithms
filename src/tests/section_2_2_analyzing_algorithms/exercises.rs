pub mod exercise_2_2_2 {
    use super::super::super::{
        super::section_2_2_analyzing_algorithms::exercises::exercise_2_2_2::selection_sort,
        utilities::run_all_sorting_tests,
    };
    use crate::test::Bencher;

    #[bench]
    pub fn test_selection_sort(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(selection_sort));
    }
}
