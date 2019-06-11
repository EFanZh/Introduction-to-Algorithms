pub trait Chip {
    fn test(&self, chip: &Self) -> bool;
}

pub fn find_one_good_chip_naive<T: Chip>(chips: &[T]) -> Option<&T> {
    fn helper<'a, T: Chip>(chips: &[&'a T]) -> Option<&'a T> {
        if chips.is_empty() {
            None
        } else {
            let filtered_chips = chips
                .chunks_exact(2)
                .filter(|pair| {
                    let chip_1 = pair[0];
                    let chip_2 = pair[1];

                    chip_1.test(chip_2) && chip_2.test(chip_1)
                })
                .map(|pair| pair[0])
                .collect::<Box<_>>();

            if chips.len() % 2 == 0 {
                helper(&filtered_chips)
            } else {
                match helper(&filtered_chips) {
                    None => chips.last().cloned(),
                    good_chip => good_chip,
                }
            }
        }
    }

    helper(&chips.iter().collect::<Box<_>>())
}

pub fn find_one_good_chip<T: Chip>(chips: &[T]) -> Option<&T> {
    let mut stack = Vec::new();

    for mut chip in chips {
        let mut level = 0;

        loop {
            if let Some((_, top_level)) = stack.last() {
                if level == *top_level {
                    let (top_chip, _) = stack.pop().unwrap();

                    if chip.test(top_chip) && top_chip.test(chip) {
                        chip = top_chip;
                        level += 1;

                        continue;
                    } else {
                        break;
                    }
                }
            }

            stack.push((chip, level));

            break;
        }
    }

    stack.first().map(|(chip, _)| *chip)
}

#[cfg(test)]
mod tests {
    use super::{find_one_good_chip, find_one_good_chip_naive, Chip};
    use rand::random;
    use rand::seq::SliceRandom;

    enum TestChip {
        Good,
        Bad,
    }

    impl Chip for TestChip {
        fn test(&self, chip: &TestChip) -> bool {
            match self {
                TestChip::Good => match chip {
                    TestChip::Good => true,
                    TestChip::Bad => false,
                },
                TestChip::Bad => random(),
            }
        }
    }

    fn run_tests(f: fn(&[TestChip]) -> Option<&TestChip>) {
        let max_num_chips = 16;
        let num_instance_tests = 10000;

        let mut rng = rand::thread_rng();
        let mut chips = Vec::new();

        for num_chips in 0..=max_num_chips {
            for num_bad_chips in 0..=num_chips / 2 {
                chips.clear();

                let num_good_chips = num_chips - num_bad_chips;

                chips.extend((0..num_bad_chips).map(|_| TestChip::Bad));
                chips.extend((0..num_good_chips).map(|_| TestChip::Good));

                for _ in 0..num_instance_tests {
                    chips.shuffle(&mut rng);

                    match f(&chips) {
                        None => assert_eq!(num_good_chips, num_bad_chips),
                        Some(TestChip::Bad) => unreachable!(),
                        Some(TestChip::Good) => (),
                    }
                }
            }
        }
    }

    #[test]
    fn test_find_one_good_chip_naive() {
        run_tests(find_one_good_chip_naive);
    }

    #[test]
    fn test_find_one_good_chip() {
        run_tests(find_one_good_chip);
    }
}
