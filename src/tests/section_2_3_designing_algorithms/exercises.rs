mod exercise_2_3_2 {
    use super::super::{
        super::{
            super::section_2_3_designing_algorithms::exercises::exercise_2_3_2::merge, utilities::run_all_sorting_tests,
        },
        Bencher,
    };

    fn merge_sort_helper_2<T: Ord + Clone>(a: &mut [T]) {
        pub fn merge_sort_2<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) {
            if r - p > 1 {
                let q = p + (r - p) / 2;

                merge_sort_2(a, p, q);
                merge_sort_2(a, q, r);
                merge(a, p, q, r);
            }
        }

        merge_sort_2(a, 0, a.len());
    }

    #[bench]
    fn test_merge_sort_2(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(merge_sort_helper_2));
    }
}

mod exercise_2_3_4 {
    use super::super::{
        super::{
            super::section_2_3_designing_algorithms::exercises::exercise_2_3_4::insertion_sort_recursive,
            utilities::run_all_sorting_tests,
        },
        Bencher,
    };

    #[bench]
    fn test_insertion_sort_recursive(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(insertion_sort_recursive));
    }
}

mod exercise_2_3_5 {
    use super::super::{
        super::{
            super::section_2_3_designing_algorithms::exercises::exercise_2_3_5::{
                binary_search_iterative, binary_search_iterative_libcxx, binary_search_iterative_rust,
                binary_search_recursive, binary_search_recursive_libcxx, binary_search_recursive_non_tail,
                binary_search_recursive_pointer, binary_search_recursive_rust,
            },
            utilities::run_all_binary_search_tests,
        },
        Bencher,
    };

    #[bench]
    fn test_binary_search_iterative(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_iterative));
    }

    #[bench]
    fn test_binary_search_iterative_libcxx(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_iterative_libcxx));
    }

    #[bench]
    fn test_binary_search_iterative_rust(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_iterative_rust));
    }

    #[bench]
    fn test_binary_search_recursive(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_recursive));
    }

    #[bench]
    fn test_binary_search_recursive_non_tail(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_recursive_non_tail));
    }

    #[bench]
    fn test_binary_search_recursive_pointer(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_recursive_pointer));
    }

    #[bench]
    fn test_binary_search_recursive_libcxx(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_recursive_libcxx));
    }

    #[bench]
    fn test_binary_search_recursive_rust(b: &mut Bencher) {
        b.iter(|| run_all_binary_search_tests(binary_search_recursive_rust));
    }
}

mod exercise_2_3_7 {
    use super::super::super::super::section_2_3_designing_algorithms::exercises::exercise_2_3_7::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[], 0), false);
        assert_eq!(two_sum(&[0], 0), false);
        assert_eq!(two_sum(&[0, 0], 0), true);
        assert_eq!(two_sum(&[0, 1], 0), false);
        assert_eq!(two_sum(&[1, 0], 1), true);
        assert_eq!(two_sum(&[0, 0, 0], 0), true);
        assert_eq!(two_sum(&[0, 0, 0], 1), false);
        assert_eq!(two_sum(&[0, 0, 1], 0), true);
        assert_eq!(two_sum(&[0, 0, 1], 1), true);
        assert_eq!(two_sum(&[0, 0, 1], 2), false);
        assert_eq!(two_sum(&[0, 1, 1], 0), false);
        assert_eq!(two_sum(&[0, 1, 1], 1), true);
        assert_eq!(two_sum(&[0, 1, 1], 2), true);
        assert_eq!(two_sum(&[0, 1, 1], 3), false);
        assert_eq!(two_sum(&[0, 1, 2], 0), false);
        assert_eq!(two_sum(&[0, 1, 2], 1), true);
        assert_eq!(two_sum(&[0, 1, 2], 2), true);
        assert_eq!(two_sum(&[0, 1, 2], 3), true);
        assert_eq!(two_sum(&[0, 1, 2], 4), false);
    }
}
