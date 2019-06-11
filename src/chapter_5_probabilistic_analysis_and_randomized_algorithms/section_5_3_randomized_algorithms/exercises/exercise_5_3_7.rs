use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub fn random_sample(m: usize, n: usize) -> HashSet<usize> {
    fn helper<R: Rng>(m: usize, n: usize, rng: &mut R) -> HashSet<usize> {
        if m == 0 {
            HashSet::new()
        } else {
            let mut s = helper(m - 1, n - 1, rng);
            let i = rng.gen_range(0, n);

            if !s.insert(i) {
                s.insert(n - 1);
            }

            s
        }
    }

    helper(m, n, &mut thread_rng())
}

pub fn random_sample_tail_recursive(m: usize, n: usize) -> HashSet<usize> {
    fn helper<R: Rng>(n: usize, mut s: HashSet<usize>, mut rng: R, k: usize) -> HashSet<usize> {
        if k > n {
            s
        } else {
            let i = rng.gen_range(0, k);

            if !s.insert(i) {
                s.insert(k - 1);
            }

            helper(n, s, rng, k + 1)
        }
    }

    helper(n, HashSet::with_capacity(m), thread_rng(), n + 1 - m)
}

pub fn random_sample_iterative(m: usize, n: usize) -> HashSet<usize> {
    let mut s = HashSet::with_capacity(m);
    let mut rng = thread_rng();

    for k in n + 1 - m..=n {
        let i = rng.gen_range(0, k);

        if !s.insert(i) {
            s.insert(k - 1);
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use num_integer::binomial;
    use std::collections::HashSet;

    fn run_random_sample_test<F: Fn(usize, usize) -> HashSet<usize>>(f: F) {
        let mut results = HashSet::new();

        for n in 0..=6 {
            for m in 0..=n {
                for _ in 0..100_000 {
                    let mut result = f(m, n).into_iter().collect::<Box<_>>();

                    result.sort_unstable();

                    assert_eq!(result.len(), m);
                    assert!(result.iter().all(|x| *x < n));

                    results.insert(result);
                }

                assert_eq!(results.len(), binomial(n, m));

                results.clear()
            }
        }
    }

    #[test]
    fn test_random_sample() {
        run_random_sample_test(super::random_sample);
    }

    #[test]
    fn test_random_sample_tail_recursive() {
        run_random_sample_test(super::random_sample_tail_recursive);
    }

    #[test]
    fn test_random_sample_iterative() {
        run_random_sample_test(super::random_sample_iterative);
    }
}
