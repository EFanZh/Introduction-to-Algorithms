pub fn random<F: FnMut() -> bool>(mut biased_random: F) -> bool {
    loop {
        let x = biased_random();
        let y = biased_random();

        if x != y {
            return x;
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Bernoulli, Distribution};

    #[test]
    fn test_random() {
        let test_count = 100_000;
        let mut rng = rand::thread_rng();

        for p_times_10 in 3..=7 {
            let distribution = Bernoulli::new(f64::from(p_times_10) / 10.0).unwrap();
            let mut biased_random = || distribution.sample(&mut rng);

            let mut diff: i32 = 0;

            for _ in 0..test_count {
                if super::random(&mut biased_random) {
                    diff += 1;
                } else {
                    diff -= 1;
                }
            }

            assert!((f64::from(diff.abs()) / f64::from(test_count)) < 0.02); // Does not guarantee to success.
        }
    }
}
