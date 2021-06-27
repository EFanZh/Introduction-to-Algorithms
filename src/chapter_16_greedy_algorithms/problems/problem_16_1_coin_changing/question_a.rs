#[derive(PartialEq, Eq, Debug)]
pub struct Coins {
    quarters: usize,
    dimes: usize,
    nickels: usize,
    pennies: usize,
}

const QUARTER_CENTS: usize = 25;
const DIME_CENTS: usize = 10;
const NICKEL_CENTS: usize = 5;
const PENNY_CENTS: usize = 1;

#[must_use]
pub fn change_coins(mut n: usize) -> Coins {
    let mut result = Coins {
        quarters: 0,
        dimes: 0,
        nickels: 0,
        pennies: 0,
    };

    for (target, value) in &mut [
        (&mut result.quarters, QUARTER_CENTS),
        (&mut result.dimes, DIME_CENTS),
        (&mut result.nickels, NICKEL_CENTS),
        (&mut result.pennies, PENNY_CENTS),
    ] {
        if n == 0 {
            break;
        }

        **target = n / *value;

        n -= *value * **target;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::change_coins;

    #[test]
    fn test_change_coins() {
        let test_cases = [
            (0, [0, 0, 0, 0]),
            (1, [0, 0, 0, 1]),
            (2, [0, 0, 0, 2]),
            (5, [0, 0, 1, 0]),
            (6, [0, 0, 1, 1]),
            (7, [0, 0, 1, 2]),
            (10, [0, 1, 0, 0]),
            (11, [0, 1, 0, 1]),
            (12, [0, 1, 0, 2]),
            (15, [0, 1, 1, 0]),
            (16, [0, 1, 1, 1]),
            (17, [0, 1, 1, 2]),
            (25, [1, 0, 0, 0]),
            (26, [1, 0, 0, 1]),
            (27, [1, 0, 0, 2]),
            (30, [1, 0, 1, 0]),
            (31, [1, 0, 1, 1]),
            (32, [1, 0, 1, 2]),
            (35, [1, 1, 0, 0]),
            (36, [1, 1, 0, 1]),
            (37, [1, 1, 0, 2]),
            (40, [1, 1, 1, 0]),
            (41, [1, 1, 1, 1]),
            (42, [1, 1, 1, 2]),
        ];

        for (n, [quarters, dimes, nickels, pennies]) in test_cases.iter().copied() {
            let result = change_coins(n);

            assert_eq!(result.quarters, quarters);
            assert_eq!(result.dimes, dimes);
            assert_eq!(result.nickels, nickels);
            assert_eq!(result.pennies, pennies);
        }
    }
}
