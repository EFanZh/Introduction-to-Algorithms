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
    use rand::distributions::Distribution;

    #[test]
    fn test_random() {
        let test_count = 100000;
        let mut p = 0.1;
        let mut rng = rand::thread_rng();

        loop {
            let distribution = rand::distributions::Bernoulli::new(p);
            let mut biased_random = || distribution.sample(&mut rng);

            let mut diff: i32 = 0;

            for _ in 0..test_count {
                if super::random(&mut biased_random) {
                    diff += 1;
                } else {
                    diff -= 1;
                }
            }

            assert!((diff.abs() as f64 / test_count as f64) < 0.02); // Does not guarantee to success.

            // Next loop.

            p += 0.1;

            if p > 0.9 {
                break;
            }
        }
    }
}
