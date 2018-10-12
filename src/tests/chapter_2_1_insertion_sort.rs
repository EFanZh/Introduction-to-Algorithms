use super::super::chapter_2_1_insertion_sort::insertion_sort;
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
}

#[test]
fn test_insertion_sort() {
    run_all_sorting_tests(insertion_sort);
}
