use super::super::extra::bucket_sort_by;

fn magnitude2((x, y): &(f64, f64)) -> f64 {
    x * x + y * y
}

pub fn bucker_sort_points(a: &mut [(f64, f64)]) {
    #[allow(clippy::cast_precision_loss)]
    let n = a.len() as f64;

    bucket_sort_by(
        a,
        |p| (n * magnitude2(p)).ceil() as usize - 1,
        |lhs, rhs| magnitude2(lhs).partial_cmp(&magnitude2(rhs)).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::{assign_vec, assign_vec_from_iter};
    use super::{bucker_sort_points, magnitude2};
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_bucket_sort_points() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..2_usize.pow(n as _) {
                assign_vec_from_iter(
                    &mut a,
                    iter::repeat_with(|| (rng.gen(), rng.gen()))
                        .filter(|p| {
                            let r2 = magnitude2(p);

                            r2 > 0.0 && r2 <= 1.0
                        })
                        .take(n),
                );

                assign_vec(&mut b, &a);

                bucker_sort_points(&mut b);

                a.sort_unstable_by(|lhs, rhs| magnitude2(lhs).partial_cmp(&magnitude2(rhs)).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
