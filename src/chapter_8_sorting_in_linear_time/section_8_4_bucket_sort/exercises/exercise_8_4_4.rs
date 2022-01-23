use super::super::extra::bucket_sort_by;

fn magnitude2((x, y): &(f64, f64)) -> f64 {
    x * x + y * y
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
pub fn bucker_sort_points(a: &mut [(f64, f64)]) {
    let n = a.len() as f64;

    bucket_sort_by(
        a,
        |p| (n * magnitude2(p)).ceil() as usize - 1,
        |lhs, rhs| magnitude2(lhs).partial_cmp(&magnitude2(rhs)).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_bucket_sort_points() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                test_utilities::assign_vec_from_iter(
                    &mut a,
                    iter::repeat_with(|| (rng.gen(), rng.gen()))
                        .filter(|p| {
                            let r2 = super::magnitude2(p);

                            r2 > 0.0 && r2 <= 1.0
                        })
                        .take(n),
                );

                test_utilities::assign_vec(&mut b, &a);

                super::bucker_sort_points(&mut b);

                a.sort_unstable_by(|lhs, rhs| super::magnitude2(lhs).partial_cmp(&super::magnitude2(rhs)).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
