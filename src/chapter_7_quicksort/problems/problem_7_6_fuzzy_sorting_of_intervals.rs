use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn compare_interval<T: Ord>(lhs: &(T, T), rhs: &(T, T)) -> Ordering {
    if lhs.1 < rhs.0 {
        Ordering::Less
    } else if lhs.0 > rhs.1 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn intersect_interval_with<T: Ord + Clone>(lhs: &mut (T, T), rhs: &(T, T)) {
    if rhs.0 > lhs.0 {
        lhs.0 = rhs.0.clone();
    }

    if rhs.1 < lhs.1 {
        lhs.1 = rhs.1.clone();
    }
}

fn fuzzy_partition<T: Clone + Ord>(a: &mut [(T, T)]) -> (usize, usize) {
    let pivot_index = a.len() - 1;
    let mut pivot = a[pivot_index].clone();

    let mut q = 0;
    let mut t = q;
    let mut u = pivot_index;

    // All elements in a[..q] < pivot.
    // All elements in a[q..t] = pivot.
    // All elements in a[t..u] is unknown.
    // All elements in a[u..pivot_index] > pivot.

    while t < u {
        let interval = &a[t];

        match compare_interval(interval, &pivot) {
            Ordering::Less => {
                a.swap(q, t);

                q += 1;
                t += 1;
            }
            Ordering::Equal => {
                intersect_interval_with(&mut pivot, interval);

                t += 1;
            }
            Ordering::Greater => {
                u -= 1;

                a.swap(t, u);
            }
        }
    }

    a.swap(t, pivot_index);

    (q, t + 1)
}

fn randomized_fuzzy_partition<T: Clone + Ord, R: Rng>(a: &mut [(T, T)], rng: &mut R) -> (usize, usize) {
    let i = rng.gen_range(0, a.len());

    a.swap(i, a.len() - 1);

    fuzzy_partition(a)
}

pub fn fuzzy_sort<T: Clone + Ord>(a: &mut [(T, T)]) {
    fn helper<T: Clone + Ord, R: Rng>(a: &mut [(T, T)], rng: &mut R) {
        if a.len() > 1 {
            let (q, t) = randomized_fuzzy_partition(a, rng);

            helper(&mut a[..q], rng);
            helper(&mut a[t..], rng);
        }
    }

    helper(a, &mut thread_rng());
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::assign_vec_from_iter;
    use super::{compare_interval, fuzzy_sort};
    use rand::{thread_rng, Rng};
    use std::cmp::Ordering;

    fn is_fuzzy_sorted<T: Ord>(a: &[(T, T)]) -> bool {
        for (j, y) in a.iter().enumerate().skip(1) {
            for x in &a[..j] {
                if compare_interval(x, y) == Ordering::Greater {
                    return false;
                }
            }
        }

        true
    }

    #[test]
    fn test_fuzzy_sort() {
        let mut rng = thread_rng();
        let mut a = Vec::new();

        for n in 0..8 {
            for _ in 0..10000 {
                assign_vec_from_iter(
                    &mut a,
                    (0..n).map(|_| {
                        let a = rng.gen_range(0, 30);
                        let len = rng.gen_range(0, 30);

                        (a, a + len)
                    }),
                );

                fuzzy_sort(&mut a);

                assert!(is_fuzzy_sorted(&a));
            }
        }
    }
}
