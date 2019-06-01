use lazy_static::lazy_static;
use permutohedron::LexicalPermutation;

fn iterate_digits<F: FnMut(&mut [i32])>(a: &mut [i32], mut f: F) {
    fn helper<F: FnMut(&mut [i32])>(a: &mut [i32], i: usize, f: &mut F) {
        if i < a.len() {
            a[i] = 0;
            helper(a, i + 1, f);
            a[i] = 1;
            helper(a, i + 1, f);
        } else {
            f(a);
        }
    }

    helper(a, 0, &mut f);
}

fn generate_ordered_sequences<F: FnMut(&mut [i32])>(sequence_storage: &mut [i32], diff_storage: &mut [i32], mut f: F) {
    let length = sequence_storage.len();

    if length < 2 {
        for n in sequence_storage.iter_mut() {
            *n = 0;
        }

        f(sequence_storage);
    } else {
        iterate_digits(&mut diff_storage[..length - 1], |diff| {
            sequence_storage[0] = 0;

            for (i, d) in diff.iter().enumerate() {
                sequence_storage[i + 1] = sequence_storage[i] + d;
            }

            f(sequence_storage);
        });
    }
}

fn generate_all_ordered_sequences<F: FnMut(&mut [i32])>(
    sequence_storage: &mut [i32],
    diff_storage: &mut [i32],
    mut f: F,
) {
    for length in 0..=sequence_storage.len() {
        generate_ordered_sequences(&mut sequence_storage[..length], diff_storage, &mut f);
    }
}

fn generate_all_unordered_sequences<F: FnMut(&mut [i32])>(
    sequence_storage: &mut [i32],
    diff_storage: &mut [i32],
    mut f: F,
) {
    generate_all_ordered_sequences(sequence_storage, diff_storage, |sequence| loop {
        f(sequence);

        if !sequence.next_permutation() {
            break;
        }
    })
}

pub fn is_max_heap<T: Ord>(a: &[T]) -> bool {
    a.iter().enumerate().skip(1).all(|(i, v)| v <= &a[(i - 1) / 2])
}

pub fn is_min_heap<T: Ord>(a: &[T]) -> bool {
    a.iter().enumerate().skip(1).all(|(i, v)| v >= &a[(i - 1) / 2])
}

const ORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH: usize = 7;
const UNORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH: usize = 7;

lazy_static! {
    static ref ORDERED_SEQUENCE_TEST_CASES: Box<[Box<[i32]>]> = {
        let mut result = Vec::new();
        let mut sequence_storage = [0; ORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH];
        let mut diff_storage = [0; ORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH - 1];

        generate_all_ordered_sequences(&mut sequence_storage, &mut diff_storage, |test_case| {
            result.push(test_case.to_vec().into_boxed_slice());
        });

        result.shrink_to_fit();

        result.into()
    };
    static ref UNORDERED_SEQUENCE_TEST_CASES: Box<[(Box<[i32]>, Box<[i32]>)]> = {
        let mut result = Vec::new();
        let mut sequence_storage = [0; UNORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH];
        let mut diff_storage = [0; UNORDERED_SEQUENCE_TEST_CASE_MAX_LENGTH - 1];

        generate_all_unordered_sequences(&mut sequence_storage, &mut diff_storage, |test_case| {
            let test_case = test_case.to_vec();
            let mut sorted_test_case = test_case.to_vec();

            sorted_test_case.sort_unstable();

            result.push((test_case.into(), sorted_test_case.into()));
        });

        result.shrink_to_fit();

        result.into()
    };
    static ref NUM_INVERSIONS_TEST_CASES: Box<[(Box<[i32]>, usize)]> = {
        let mut result = Vec::with_capacity(UNORDERED_SEQUENCE_TEST_CASES.len());

        for (sequence, _) in UNORDERED_SEQUENCE_TEST_CASES.iter() {
            let mut num_inversions = 0;

            for i in (1..sequence.len()).map(|x| x - 1) {
                for j in i + 1..sequence.len() {
                    if sequence[i] > sequence[j] {
                        num_inversions += 1;
                    }
                }
            }

            result.push((sequence.clone(), num_inversions));
        }

        result.into()
    };
    static ref MAX_HEAP_TEST_CASES: Box<[Box<[i32]>]> = UNORDERED_SEQUENCE_TEST_CASES
        .iter()
        .filter_map(|(sequence, _)| {
            if is_max_heap(sequence) {
                Some(sequence.clone())
            } else {
                None
            }
        })
        .collect();
}

pub fn assign_vec<T: Clone>(target: &mut Vec<T>, source: &[T]) {
    target.clear();
    target.extend_from_slice(source);
}

pub fn assign_vec_from_iter<T, I: IntoIterator<Item = T>>(target: &mut Vec<T>, source: I) {
    target.clear();

    for value in source {
        target.push(value);
    }
}

pub fn loop_on_all_unordered_sequences<F: FnMut(&[i32], &[i32])>(mut f: F) {
    for (sequence, sorted_sequence) in UNORDERED_SEQUENCE_TEST_CASES.iter() {
        f(sequence, sorted_sequence);
    }
}

pub fn run_all_sorting_tests<S: FnMut(&mut [i32])>(mut sorter: S) {
    let mut a = Vec::new();

    loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
        assign_vec(&mut a, sequence);

        sorter(&mut a);

        assert_eq!(a.as_slice(), sorted_sequence.as_ref());
    });
}

pub fn run_all_reverse_sorting_tests<S: Fn(&mut [i32])>(sorter: S) {
    let mut a = Vec::new();

    for (test_case, expected) in UNORDERED_SEQUENCE_TEST_CASES.iter() {
        assign_vec(&mut a, test_case);

        sorter(&mut a);

        assert!(a.iter().rev().eq(expected.as_ref()));
    }
}

pub fn run_all_binary_search_tests<S: Fn(&[i32], &i32) -> Option<usize>>(searcher: S) {
    for test_case in ORDERED_SEQUENCE_TEST_CASES.iter() {
        if test_case.is_empty() {
            assert!(searcher(test_case, &0).is_none());
        } else {
            assert!(searcher(test_case, &-1).is_none());

            let max = *test_case.last().unwrap();

            for target in 0..=max {
                assert!(test_case[searcher(test_case, &target).unwrap()] == target);
            }

            assert!(searcher(test_case, &(max + 1)).is_none());
        }
    }
}

pub fn run_all_num_inversions_tests<F: Fn(&[i32]) -> usize>(num_inversions: F) {
    for (test_case, expected) in NUM_INVERSIONS_TEST_CASES.iter() {
        assert_eq!(num_inversions(test_case), *expected);
    }
}

pub fn loop_on_all_max_heap_test_cases<F: FnMut(&[i32])>(f: F) {
    MAX_HEAP_TEST_CASES.iter().map(|x| x.as_ref()).for_each(f);
}

mod tests {
    use super::{generate_all_ordered_sequences, generate_all_unordered_sequences};
    use num_integer::binomial;
    use std::collections::HashSet;

    // TODO: Use `slice::is_sorted` instead.

    fn is_sorted<T: Ord>(a: &[T]) -> bool {
        return a.windows(2).all(|pair| pair[0] <= pair[1]);
    }

    #[test]
    fn test_generate_all_ordered_sequences() {
        const MAX_MAX_LENGTH: usize = 5;

        let mut sequence_storage = vec![0; MAX_MAX_LENGTH];
        let mut diff_storage = vec![0; MAX_MAX_LENGTH - 1];
        let mut result = HashSet::new();

        for max_length in 0..=MAX_MAX_LENGTH {
            result.clear();

            generate_all_ordered_sequences(&mut sequence_storage[..max_length], &mut diff_storage, |sequence| {
                // Correct length.

                assert!(sequence.len() <= max_length);

                // Ordered.

                assert!(is_sorted(sequence));

                // Starts with zero.

                assert!(sequence.first().iter().all(|&&x| x == 0));

                // Only increase by zero or one.

                assert!(sequence
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .all(|x| x == 0 || x == 1));

                assert!(result.insert(sequence.to_vec()));
            });

            if max_length == 0 {
                assert_eq!(result.len(), 1);
            } else {
                assert_eq!(result.len(), usize::pow(2, max_length as u32));
            }
        }
    }

    // https://oeis.org/A000670

    fn fubini(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            (1..=n).map(|k| binomial(n, k) * fubini(n - k)).sum()
        }
    }

    #[test]
    fn test_fubini() {
        assert_eq!(fubini(0), 1);
        assert_eq!(fubini(1), 1);
        assert_eq!(fubini(2), 3);
        assert_eq!(fubini(3), 13);
        assert_eq!(fubini(4), 75);
        assert_eq!(fubini(5), 541);
        assert_eq!(fubini(6), 4683);
        assert_eq!(fubini(7), 47293);
    }

    // https://oeis.org/A217389

    fn fubini_partial_sums(n: usize) -> usize {
        (0..=n).map(fubini).sum()
    }

    #[test]
    fn test_fubini_partial_sums() {
        assert_eq!(fubini_partial_sums(0), 1);
        assert_eq!(fubini_partial_sums(1), 2);
        assert_eq!(fubini_partial_sums(2), 5);
        assert_eq!(fubini_partial_sums(3), 18);
        assert_eq!(fubini_partial_sums(4), 93);
        assert_eq!(fubini_partial_sums(5), 634);
        assert_eq!(fubini_partial_sums(6), 5317);
        assert_eq!(fubini_partial_sums(7), 52610);
    }

    #[test]
    fn test_generate_all_unordered_test_cases() {
        const MAX_MAX_LENGTH: usize = 5;

        let mut sequence_storage = vec![0; MAX_MAX_LENGTH];
        let mut diff_storage = vec![0; MAX_MAX_LENGTH - 1];
        let mut result = HashSet::new();

        for max_length in 0..=MAX_MAX_LENGTH {
            result.clear();

            generate_all_unordered_sequences(&mut sequence_storage[..max_length], &mut diff_storage, |sequence| {
                // Correct length.

                assert!(sequence.len() <= max_length);

                // Store the result.

                let saved_value = sequence.to_vec();

                // Starts with zero.

                sequence.sort_unstable();

                assert!(sequence.first().iter().all(|&&x| x == 0));

                // Only increase by zero or one.

                assert!(sequence
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .all(|x| x == 0 || x == 1));

                // Restore the original value.

                sequence.copy_from_slice(&saved_value);

                assert!(result.insert(saved_value));
            });

            assert_eq!(result.len(), fubini_partial_sums(max_length));
        }
    }
}
