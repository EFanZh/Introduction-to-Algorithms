use super::super::extra;
use std::cmp::Ordering;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)] // Expected.
pub fn bucker_sort_by_probability_distribution<T: Clone, F: FnMut(&T, &T) -> Ordering, P: FnMut(&T) -> f64>(
    a: &mut [T],
    compare: F,
    mut p: P,
) {
    let n = a.len() as f64;

    extra::bucket_sort_by(a, |x| (n * p(x)).ceil() as usize - 1, compare);
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_bucker_sort_by_probability_distribution() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                test_utilities::assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen::<f64>()).take(n));

                test_utilities::assign_vec(&mut b, &a);

                super::bucker_sort_by_probability_distribution(
                    &mut b,
                    |lhs, rhs| lhs.partial_cmp(rhs).unwrap(),
                    |&x| x,
                );

                a.sort_unstable_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
