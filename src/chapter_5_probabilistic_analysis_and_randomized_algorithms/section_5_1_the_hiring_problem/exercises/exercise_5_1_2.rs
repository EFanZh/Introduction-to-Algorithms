use rand::Rng;

pub fn random(a: i32, b: i32) -> i32 {
    let range = b - a;
    let bits_needed = f64::from(b - a + 1).log2().ceil() as _;
    let mut rng = rand::thread_rng();

    loop {
        let mut result = 0;

        for _ in 0..bits_needed {
            result <<= 1;

            result |= rng.gen::<bool>() as i32;
        }

        if result <= range {
            return a + result;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    const _TEST_RANGE: i32 = 16;
    const _TEST_SAMPLES: i32 = 10000;

    #[test]
    fn test_random_range() {
        for start in -_TEST_RANGE..=_TEST_RANGE {
            for end in start..=_TEST_RANGE {
                for _ in 0.._TEST_SAMPLES {
                    let r = super::random(start, end);

                    assert!(r >= start);
                    assert!(r <= end);
                }
            }
        }
    }

    #[test]
    fn test_random_coverage() {
        let mut set = HashSet::new();

        for start in -_TEST_RANGE..=_TEST_RANGE {
            for end in start..=_TEST_RANGE {
                set.extend(start..=end);

                loop {
                    set.remove(&super::random(start, end));

                    if set.is_empty() {
                        break;
                    }
                }
            }
        }
    }
}
