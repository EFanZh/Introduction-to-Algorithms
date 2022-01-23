#[must_use]
pub fn plan_investment_strategy(
    return_rates: &[f64],
    years: usize,
    keep_fee: f64,
    switch_fee: f64,
    money: f64,
) -> (f64, Box<[usize]>) {
    let num_strategies = return_rates.len() / years;
    let mut cache = vec![(0.0, None); return_rates.len()];

    // Get first year best strategies.

    for (choice, return_rate) in cache.iter_mut().zip(return_rates).take(num_strategies) {
        choice.0 = money * return_rate;
    }

    // Get rest years best strategies.

    for (year, year_return_rates) in return_rates.chunks(num_strategies).enumerate().skip(1) {
        let (i, (mut fund_if_switch, _)) = cache[num_strategies * (year - 1)..num_strategies * year]
            .iter()
            .enumerate()
            .max_by(|(_, (lhs, _)), (_, (rhs, _))| lhs.partial_cmp(rhs).unwrap())
            .unwrap();

        fund_if_switch -= switch_fee;

        for (strategy, return_rate) in year_return_rates.iter().enumerate() {
            let fund_if_keep = cache[num_strategies * (year - 1) + strategy].0 - keep_fee;

            let mut choice = if fund_if_switch > fund_if_keep {
                (fund_if_switch, Some(i))
            } else {
                (fund_if_keep, Some(strategy))
            };

            choice.0 *= *return_rate;

            cache[num_strategies * year + strategy] = choice;
        }
    }

    // Build result;

    let mut result = vec![0; years];

    let (mut strategy, (fund, _)) = cache[cache.len() - num_strategies..]
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, lhs), (_, rhs)| lhs.0.partial_cmp(&rhs.0).unwrap())
        .unwrap();

    for (r, choices) in result.iter_mut().zip(cache.chunks_exact(num_strategies)).rev() {
        *r = strategy;

        if let Some(previous_strategy) = choices[strategy].1 {
            strategy = previous_strategy;
        }
    }

    (fund, result.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plan_investment_strategy() {
        let return_rates = [
            1.043, 1.042, 1.044, 1.041, 1.037, 1.031, 1.046, //
            1.037, 1.041, 1.047, 1.032, 1.047, 1.048, 1.040, //
            1.040, 1.048, 1.033, 1.041, 1.050, 1.048, 1.050, //
            1.036, 1.031, 1.044, 1.049, 1.044, 1.038, 1.033, //
            1.049, 1.048, 1.036, 1.035, 1.031, 1.044, 1.037, //
            1.035, 1.042, 1.036, 1.038, 1.048, 1.036, 1.043, //
            1.042, 1.047, 1.041, 1.045, 1.039, 1.048, 1.032, //
            1.046, 1.037, 1.037, 1.049, 1.034, 1.047, 1.050, //
            1.030, 1.049, 1.037, 1.037, 1.049, 1.032, 1.046, //
            1.039, 1.036, 1.049, 1.035, 1.033, 1.049, 1.048,
        ];

        let years = 10;
        let keep_fee = 50.0;
        let switch_fee = 100.0;
        let initial_money = 10000.0;

        let (fund, strategies) =
            super::plan_investment_strategy(&return_rates, years, keep_fee, switch_fee, initial_money);

        approx::assert_ulps_eq!(fund, 15_035.294_028_598_65);
        assert_eq!(*strategies, [6, 4, 4, 3, 1, 1, 1, 6, 6, 6]);
    }
}
