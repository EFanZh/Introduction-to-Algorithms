use rand::{thread_rng, Rng};

// Permute-By-Sorting
//
// 1  n = A.length
// 2  let P[1..n] be a new array
// 3  for i = 1 to n
// 4      P[i] = Random(1, n³)
// 5  sort A, using P as sort keys

pub fn permute_by_sorting<T: Clone>(a: &mut [T]) {
    let n = a.len();
    let cubed_n = n * n * n;
    let mut rng = thread_rng();

    let p = (0..n).map(|_| rng.gen_range(0, cubed_n));

    // Have to use Vec<_>: https://github.com/rust-lang/rust/issues/25725

    let mut a_and_p = a
        .iter()
        .zip(p)
        .map(|(a_item, p_item)| (a_item.clone(), p_item))
        .collect::<Vec<_>>();

    // Have to dereference here: https://stackoverflow.com/questions/47121985/why-cant-i-use-a-key-function-that-returns-a-reference-when-sorting-a-vector-wi.

    a_and_p.sort_unstable_by_key(|(_, key)| *key);

    for (a_item, (a_shuffled, _)) in a.iter_mut().zip(a_and_p) {
        *a_item = a_shuffled;
    }
}

// Randomize-In-Place
//
// 1  n - A.length
// 2  for i = 1 to n
// 3      swap A[i] with A[Random(i, n)]

pub fn randomize_in_place<T>(a: &mut [T]) {
    let n = a.len();
    let mut rng = thread_rng();

    for i in 0..n {
        a.swap(i, rng.gen_range(i, n));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn run_test_randomize<F: Fn(&mut [i32])>(f: F) {
        let mut a = Vec::new();
        let mut a_sorted = Vec::new();
        let mut permutation_counter = HashMap::<Vec<i32>, usize>::new();
        let mut expected_permutations = 1;

        for n in 0..=6 {
            for _ in 0..100_000 {
                f(&mut a);

                permutation_counter
                    .entry(a.clone())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);


                a.sort_unstable();

                assert_eq!(a, a_sorted);
            }

            assert_eq!(permutation_counter.len(), expected_permutations);

            a.push(n);
            a_sorted.push(n);
            permutation_counter.clear();
            expected_permutations *= (n + 1) as usize;
        }
    }

    #[test]
    fn test_permute_by_sorting() {
        run_test_randomize(super::permute_by_sorting);
    }

    #[test]
    fn test_randomize_in_place() {
        run_test_randomize(super::randomize_in_place);
    }
}
