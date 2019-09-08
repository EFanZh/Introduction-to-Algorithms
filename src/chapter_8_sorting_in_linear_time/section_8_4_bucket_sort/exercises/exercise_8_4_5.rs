use super::super::extra::bucket_sort_by;
use std::cmp::Ordering;

pub fn bucker_sort_by_probability_distribution<T: Clone, F: FnMut(&T, &T) -> Ordering, P: FnMut(&T) -> f64>(
    a: &mut [T],
    compare: F,
    mut p: P,
) {
    let n = a.len() as f64;

    bucket_sort_by(a, |x| (n * p(x)).ceil() as usize - 1, compare);
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::{assign_vec, assign_vec_from_iter};
    use super::bucker_sort_by_probability_distribution;
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_bucker_sort_by_probability_distribution() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0usize..10 {
            for _ in 0..2usize.pow(n as _) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen::<f64>()).take(n));

                assign_vec(&mut b, &a);

                bucker_sort_by_probability_distribution(&mut b, |lhs, rhs| lhs.partial_cmp(rhs).unwrap(), |&x| x);

                a.sort_unstable_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
