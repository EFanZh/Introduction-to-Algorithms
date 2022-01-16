use crate::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice_by;
use std::cmp::Ordering;

pub fn bucket_sort_by<T: Clone, F: FnMut(&T) -> usize, G: FnMut(&T, &T) -> Ordering>(
    a: &mut [T],
    mut f: F,
    mut compare: G,
) {
    let n = a.len();
    let mut b = vec![Vec::new(); n];

    for a_i in &*a {
        b[f(a_i)].push(a_i.clone());
    }

    for b_i in &mut b {
        insertion_sort_slice_by(b_i, &mut compare);
    }

    for (lhs, rhs) in a.iter_mut().zip(b.into_iter().flatten()) {
        *lhs = rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::bucket_sort_by;
    use crate::test_utilities::{assign_vec, assign_vec_from_iter};
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_bucket_sort_by() {
        let mut a = Vec::<f64>::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen()).take(n));
                assign_vec(&mut b, &a);

                #[allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_precision_loss,
                    clippy::cast_sign_loss
                )]
                bucket_sort_by(
                    &mut b,
                    |x| ((n as f64) * x) as usize,
                    |lhs, rhs| lhs.partial_cmp(rhs).unwrap(),
                );

                a.sort_unstable_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
