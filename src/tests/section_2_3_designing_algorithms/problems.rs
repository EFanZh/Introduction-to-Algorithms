mod problem_2_2 {
    use super::super::{
        super::{
            super::section_2_3_designing_algorithms::problems::problem_2_2::bubble_sort,
            utilities::run_all_sorting_tests,
        },
        Bencher,
    };

    #[bench]
    fn test_bubble_sort(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(bubble_sort));
    }
}
