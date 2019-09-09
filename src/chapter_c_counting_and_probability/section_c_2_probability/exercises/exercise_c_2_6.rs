use std::cmp::Ordering;

pub fn flip_coin<C: FnMut() -> bool>(mut fair_coin: C, mut a: i32, b: i32) -> bool {
    loop {
        let bit = a * 2 < b;

        if fair_coin() != bit {
            return !bit;
        } else {
            a = (a * 2) % b;
        }
    }
}

pub fn flip_coin_2<C: FnMut() -> bool>(mut fair_coin: C, a: i32, b: i32) -> bool {
    let flip_result = fair_coin();

    match (a * 2).cmp(&b) {
        Ordering::Less => flip_result && flip_coin_2(fair_coin, a * 2, b),
        Ordering::Equal => flip_result,
        Ordering::Greater => flip_result || flip_coin_2(fair_coin, a * 2 - b, b),
    }
}

#[cfg(test)]
mod tests {
    use super::{flip_coin, flip_coin_2};
    use rand::{thread_rng, Rng};

    fn run_test_flip_coin<F: FnMut(i32, i32) -> bool>(mut f: F) {
        let samples = 100_000;

        for a in 1..=5 {
            for b in a + 1..=6 {
                let heads = (0..samples).map(|_| f(a, b)).filter(|&x| x).count() as i32;

                assert!((f64::from(b * heads) / f64::from(a * samples) - 1.0).abs() < 0.03);
            }
        }
    }

    #[test]
    fn test_flip_coin() {
        let mut rng = thread_rng();
        let mut fair_coin = || rng.gen();

        run_test_flip_coin(|a, b| flip_coin(&mut fair_coin, a, b));
    }

    #[test]
    fn test_flip_coin_2() {
        let mut rng = thread_rng();
        let mut fair_coin = || rng.gen();

        run_test_flip_coin(|a, b| flip_coin_2(&mut fair_coin, a, b));
    }
}
