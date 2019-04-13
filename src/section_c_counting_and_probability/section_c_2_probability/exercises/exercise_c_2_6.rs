use std::cmp::Ordering;

pub fn flip_coin<C: FnMut() -> bool>(mut fair_coin: C, a: i32, b: i32) -> bool {
    let flip_result = fair_coin();

    match (a * 2).cmp(&b) {
        Ordering::Less => flip_result && flip_coin(fair_coin, a * 2, b),
        Ordering::Equal => flip_result,
        Ordering::Greater => flip_result || flip_coin(fair_coin, a * 2 - b, b),
    }
}

#[cfg(test)]
mod tests {
    use super::flip_coin;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_flip_coin() {
        let samples = 100_000;
        let mut rng = thread_rng();

        for a in 1..=5 {
            for b in a + 1..=6 {
                let heads = (0..samples)
                    .map(|_| flip_coin(|| rng.gen(), a, b))
                    .filter(|x| *x)
                    .count() as i32;

                assert!((f64::from(a * samples) / f64::from(b * heads) - 1.0).abs() < 0.01);
            }
        }
    }
}
