use super::{super::section_2_1_insertion_sort::insertion_sort, utilities::run_all_sorting_tests};
use crate::test::Bencher;

#[bench]
fn test_insertion_sort(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(insertion_sort));
}

mod extra {
    use super::{
        super::super::section_2_1_insertion_sort::extra::insertion_sort_recursive, run_all_sorting_tests, Bencher,
    };

    #[bench]
    fn test_insertion_sort_2(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(insertion_sort_recursive));
    }
}

mod exercises {
    mod exercise_2_1_2 {
        use super::super::super::{
            super::section_2_1_insertion_sort::exercises::exercise_2_1_2::insertion_sort_reversed, utilities::test_sort,
        };

        #[test]
        fn test_exercise_insertion_sort_reversed() {
            test_sort(insertion_sort_reversed, [] as [i32; 0], []);

            test_sort(insertion_sort_reversed, [0], [0]);

            test_sort(insertion_sort_reversed, [0, 0], [0, 0]);
            test_sort(insertion_sort_reversed, [0, 1], [1, 0]);
            test_sort(insertion_sort_reversed, [1, 0], [1, 0]);

            test_sort(insertion_sort_reversed, [0, 0, 0], [0, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 1], [1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 0], [1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 1], [1, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 2], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 0], [1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 1], [1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 0], [1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0], [2, 1, 0]);

            test_sort(insertion_sort_reversed, [0, 0, 0, 0], [0, 0, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 0, 1], [1, 0, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 1, 0], [1, 0, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 1, 1], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 1, 2], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 0, 2, 1], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 0, 0], [1, 0, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 0, 1], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 0, 2], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 1, 0], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 1, 1], [1, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 1, 2], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 2, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 2, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 2, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 2, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 3, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 0, 1], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 2, 1], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 3, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 3, 1, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 3, 2, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 0, 0], [1, 0, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 0, 1], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 0, 2], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 1, 0], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 1, 1], [1, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 1, 2], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 3, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 0, 0], [1, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 0, 1], [1, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 0, 2], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 1, 0], [1, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 1, 2, 0], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 1, 0], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 2, 0], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 3, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 3, 0, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 3, 2, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 0, 1], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 2, 1], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 3, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0, 0], [2, 1, 0, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0, 1], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0, 2], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 1, 0], [2, 1, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 2, 0], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 3, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 2, 0, 1], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 2, 1, 0], [2, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 3, 0, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 3, 1, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 0, 1, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 0, 2, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 1, 0, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 1, 2, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 2, 0, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [3, 2, 1, 0], [3, 2, 1, 0]);
        }
    }

    mod exercise_2_1_3 {
        use super::super::super::super::section_2_1_insertion_sort::exercises::exercise_2_1_3::search;

        #[test]
        fn test_search() {
            assert_eq!(search(&[] as &[i32; 0], &1), None);

            assert_eq!(search(&[0], &1), None);
            assert_eq!(search(&[1], &1), Some(0));

            assert_eq!(search(&([0, 0]), &1), None);
            assert_eq!(search(&([0, 1]), &1), Some(1));
            assert_eq!(search(&([1, 0]), &1), Some(0));
            assert_eq!(search(&([1, 1]), &1), Some(0));
        }
    }

    mod exercise_2_1_4 {
        use super::super::super::super::section_2_1_insertion_sort::exercises::exercise_2_1_4::add_binary;

        fn run_add_binary_test<T: AsRef<[u8]>, U: AsRef<[u8]> + AsMut<[u8]> + Default>(a: T, b: T, expected: U) {
            let mut result: U = Default::default();

            add_binary(a.as_ref(), b.as_ref(), result.as_mut());

            assert_eq!(*result.as_ref(), *expected.as_ref());
        }

        #[test]
        fn test_add_binary() {
            run_add_binary_test([], [], [0]);

            run_add_binary_test([0], [0], [0, 0]);
            run_add_binary_test([0], [1], [0, 1]);
            run_add_binary_test([1], [0], [0, 1]);
            run_add_binary_test([1], [1], [1, 0]);

            run_add_binary_test([0, 0], [0, 0], [0, 0, 0]);
            run_add_binary_test([0, 0], [0, 1], [0, 0, 1]);
            run_add_binary_test([0, 0], [1, 0], [0, 1, 0]);
            run_add_binary_test([0, 0], [1, 1], [0, 1, 1]);
            run_add_binary_test([0, 1], [0, 0], [0, 0, 1]);
            run_add_binary_test([0, 1], [0, 1], [0, 1, 0]);
            run_add_binary_test([0, 1], [1, 0], [0, 1, 1]);
            run_add_binary_test([0, 1], [1, 1], [1, 0, 0]);
            run_add_binary_test([1, 0], [0, 0], [0, 1, 0]);
            run_add_binary_test([1, 0], [0, 1], [0, 1, 1]);
            run_add_binary_test([1, 0], [1, 0], [1, 0, 0]);
            run_add_binary_test([1, 0], [1, 1], [1, 0, 1]);
            run_add_binary_test([1, 1], [0, 0], [0, 1, 1]);
            run_add_binary_test([1, 1], [0, 1], [1, 0, 0]);
            run_add_binary_test([1, 1], [1, 0], [1, 0, 1]);
            run_add_binary_test([1, 1], [1, 1], [1, 1, 0]);
        }
    }
}
