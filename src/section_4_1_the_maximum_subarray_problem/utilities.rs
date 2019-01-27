pub fn run_find_maximum_subarray_tests<F: Fn(&[i32]) -> (usize, usize, i32)>(f: F) {
    fn run_test<T: AsRef<[i32]>, F: Fn(&[i32]) -> (usize, usize, i32)>(f: F, a: T, expected: i32) {
        let a_ref = a.as_ref();

        assert!(!a_ref.is_empty());

        let (left, right, sum) = f(a_ref);

        assert!(right > left);

        assert_eq!(sum, a_ref[left..right].iter().sum());
        assert_eq!(sum, expected);
    }

    run_test(&f, [0], 0);
    run_test(&f, [-3], -3);
    run_test(&f, [4], 4);

    run_test(&f, [-3, -1], -1);
    run_test(&f, [-1, -3], -1);
    run_test(&f, [-3, 0], 0);
    run_test(&f, [-3, 3], 3);
    run_test(&f, [0, -3], 0);
    run_test(&f, [0, 0], 0);
    run_test(&f, [0, 3], 3);
    run_test(&f, [3, -1], 3);
    run_test(&f, [3, 0], 3);
    run_test(&f, [3, 4], 7);
    run_test(&f, [4, 3], 7);

    run_test(&f, [-7, -3, -1], -1);
    run_test(&f, [-7, -1, -3], -1);
    run_test(&f, [-3, -7, -1], -1);
    run_test(&f, [-3, -1, -7], -1);
    run_test(&f, [-1, -7, -3], -1);
    run_test(&f, [-1, -3, -7], -1);
    run_test(&f, [-7, -3, 1], 1);
    run_test(&f, [-7, 1, -3], 1);
    run_test(&f, [-3, -7, 1], 1);
    run_test(&f, [-3, 1, -7], 1);
    run_test(&f, [1, -7, -3], 1);
    run_test(&f, [1, -3, -7], 1);
    run_test(&f, [-7, 3, 1], 4);
    run_test(&f, [-7, 1, 3], 4);
    run_test(&f, [3, -7, 1], 3);
    run_test(&f, [3, 1, -7], 4);
    run_test(&f, [1, -7, 3], 3);
    run_test(&f, [1, 3, -7], 4);

    run_test(&f, [2, -1, 4], 5);
    run_test(&f, [2, 1, 4], 7);
    run_test(&f, [2, 0, 4], 6);
    run_test(&f, [2, 4, 0], 6);

    // Test cases from LeetCode (https://leetcode.com/problems/maximum-subarray/).

    run_test(&f, [-2, 1, -3, 4, -1, 2, 1, -5, 4], 6);
}
