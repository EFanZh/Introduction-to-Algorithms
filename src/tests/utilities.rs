use std::fmt::Debug;

pub fn test_sort<T: Ord + Clone, U: AsMut<[T]> + Debug + Eq, S: Fn(&mut [T]) -> ()>(
    sorter: S,
    mut input: U,
    expected: U,
) {
    sorter(input.as_mut());

    assert_eq!(input, expected);
}

pub fn run_all_sorting_tests<S: Fn(&mut [i32]) -> ()>(sorter: S) {
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
