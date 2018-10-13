use super::super::chapter_2_1_insertion_sort::insertion_sort;
use crate::test::Bencher;
use std::convert::AsMut;

fn test_sort<T: Ord + Clone, U: AsMut<[T]> + std::fmt::Debug + Eq, S: Fn(&mut [T]) -> ()>(
    sorter: S,
    mut input: U,
    expected: U,
) {
    sorter(input.as_mut());

    assert_eq!(input, expected);
}

fn run_all_sorting_tests<S: Fn(&mut [i32]) -> ()>(sorter: S) {
    let sorter_ref = &sorter;

    test_sort(sorter_ref, [], []);

    test_sort(sorter_ref, [0], [0]);

    test_sort(sorter_ref, [0, 1], [0, 1]);
    test_sort(sorter_ref, [1, 0], [0, 1]);

    test_sort(sorter_ref, [0, 1, 2], [0, 1, 2]);
    test_sort(sorter_ref, [0, 2, 1], [0, 1, 2]);
    test_sort(sorter_ref, [1, 0, 2], [0, 1, 2]);
    test_sort(sorter_ref, [1, 2, 0], [0, 1, 2]);
    test_sort(sorter_ref, [2, 0, 1], [0, 1, 2]);
    test_sort(sorter_ref, [2, 1, 0], [0, 1, 2]);

    test_sort(sorter_ref, [0, 1, 2, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [0, 1, 3, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [0, 2, 1, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [0, 2, 3, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [0, 3, 1, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [0, 3, 2, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 0, 2, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 0, 3, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 2, 0, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 2, 3, 0], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 3, 0, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [1, 3, 2, 0], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 0, 1, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 0, 3, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 1, 0, 3], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 1, 3, 0], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 3, 0, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [2, 3, 1, 0], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 0, 1, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 0, 2, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 1, 0, 2], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 1, 2, 0], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 2, 0, 1], [0, 1, 2, 3]);
    test_sort(sorter_ref, [3, 2, 1, 0], [0, 1, 2, 3]);

    test_sort(sorter_ref, [0, 1, 2, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 1, 2, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 1, 3, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 1, 3, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 1, 4, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 1, 4, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 1, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 1, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 3, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 3, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 4, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 2, 4, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 1, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 1, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 2, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 2, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 4, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 3, 4, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 1, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 1, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 2, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 2, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 3, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [0, 4, 3, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 2, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 2, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 3, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 3, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 4, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 0, 4, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 0, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 0, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 3, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 3, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 4, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 2, 4, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 0, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 0, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 2, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 2, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 4, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 3, 4, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 0, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 0, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 2, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 2, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 3, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [1, 4, 3, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 1, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 1, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 3, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 3, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 4, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 0, 4, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 0, 3, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 0, 4, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 3, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 3, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 4, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 1, 4, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 0, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 0, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 1, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 1, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 4, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 3, 4, 1, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 0, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 0, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 1, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 1, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 3, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [2, 4, 3, 1, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 1, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 1, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 2, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 2, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 4, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 0, 4, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 0, 2, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 0, 4, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 2, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 2, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 4, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 1, 4, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 0, 1, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 0, 4, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 1, 0, 4], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 1, 4, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 4, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 2, 4, 1, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 0, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 0, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 1, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 1, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 2, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [3, 4, 2, 1, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 1, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 1, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 2, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 2, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 3, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 0, 3, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 0, 2, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 0, 3, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 2, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 2, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 3, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 1, 3, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 0, 1, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 0, 3, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 1, 0, 3], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 1, 3, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 3, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 2, 3, 1, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 0, 1, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 0, 2, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 1, 0, 2], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 1, 2, 0], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 2, 0, 1], [0, 1, 2, 3, 4]);
    test_sort(sorter_ref, [4, 3, 2, 1, 0], [0, 1, 2, 3, 4]);
}

#[bench]
fn test_insertion_sort(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(insertion_sort));
}

mod extra {
    use super::{
        super::super::chapter_2_1_insertion_sort::extra::insertion_sort_recursive, run_all_sorting_tests, Bencher,
    };

    #[bench]
    fn test_insertion_sort_2(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(insertion_sort_recursive));
    }
}

mod exercises {
    mod exercise_2_1_2 {
        use super::super::{
            super::super::chapter_2_1_insertion_sort::exercises::exercise_2_1_2::insertion_sort_reversed, test_sort,
        };

        #[test]
        fn test_exercise_insertion_sort_reversed() {
            test_sort(insertion_sort_reversed, [] as [i32; 0], []);

            test_sort(insertion_sort_reversed, [0], [0]);

            test_sort(insertion_sort_reversed, [0, 1], [1, 0]);
            test_sort(insertion_sort_reversed, [1, 0], [1, 0]);

            test_sort(insertion_sort_reversed, [0, 1, 2], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1], [2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0], [2, 1, 0]);

            test_sort(insertion_sort_reversed, [0, 1, 2, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 1, 3, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 1, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 2, 3, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 3, 1, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [0, 3, 2, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 2, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 0, 3, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 0, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 2, 3, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 3, 0, 2], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [1, 3, 2, 0], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 1, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 0, 3, 1], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 0, 3], [3, 2, 1, 0]);
            test_sort(insertion_sort_reversed, [2, 1, 3, 0], [3, 2, 1, 0]);
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
        use super::super::super::super::chapter_2_1_insertion_sort::exercises::exercise_2_1_3::search;

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
        use super::super::super::super::chapter_2_1_insertion_sort::exercises::exercise_2_1_4::add_binary;

        fn run_test<T: AsRef<[u8]>, U: AsRef<[u8]> + AsMut<[u8]> + Default>(a: T, b: T, expected: U) {
            let mut result: U = Default::default();

            add_binary(a.as_ref(), b.as_ref(), result.as_mut());

            assert_eq!(*result.as_ref(), *expected.as_ref());
        }

        #[test]
        fn test_add_binary() {
            run_test([], [], [0]);

            run_test([0], [0], [0, 0]);
            run_test([0], [1], [0, 1]);
            run_test([1], [0], [0, 1]);
            run_test([1], [1], [1, 0]);

            run_test([0, 0], [0, 0], [0, 0, 0]);
            run_test([0, 0], [0, 1], [0, 0, 1]);
            run_test([0, 0], [1, 0], [0, 1, 0]);
            run_test([0, 0], [1, 1], [0, 1, 1]);
            run_test([0, 1], [0, 0], [0, 0, 1]);
            run_test([0, 1], [0, 1], [0, 1, 0]);
            run_test([0, 1], [1, 0], [0, 1, 1]);
            run_test([0, 1], [1, 1], [1, 0, 0]);
            run_test([1, 0], [0, 0], [0, 1, 0]);
            run_test([1, 0], [0, 1], [0, 1, 1]);
            run_test([1, 0], [1, 0], [1, 0, 0]);
            run_test([1, 0], [1, 1], [1, 0, 1]);
            run_test([1, 1], [0, 0], [0, 1, 1]);
            run_test([1, 1], [0, 1], [1, 0, 0]);
            run_test([1, 1], [1, 0], [1, 0, 1]);
            run_test([1, 1], [1, 1], [1, 1, 0]);
        }
    }
}
