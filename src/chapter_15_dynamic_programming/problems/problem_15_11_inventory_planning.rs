fn get_total_demands(values: &[usize]) -> Box<[usize]> {
    let mut result = vec![0; values.len()];
    let mut sum = 0;

    for (item, value) in result.iter_mut().zip(values).rev() {
        sum += value;

        *item = sum;
    }

    result.into()
}

pub fn plan_inventory<F: FnMut(usize) -> u64>(
    demands: &[usize],
    base_month_yield: usize,
    extra_cost_per_machine: u64,
    mut holding_cost: F,
) -> Box<[usize]> {
    let n = demands.len();
    let mut result = vec![0; n];
    let total_demands = get_total_demands(demands);

    if let Some(&total_demand) = total_demands.first() {
        let cache_columns = total_demand + 1;
        let mut cache = vec![(0, 0); cache_columns * n];

        // Initialize the last month.

        let last_month_start = cache.len() - cache_columns;
        let last_month_demand = demands[n - 1];

        for (stock, choice) in cache[last_month_start..=last_month_start + last_month_demand]
            .iter_mut()
            .enumerate()
        {
            let month_yield = last_month_demand - stock;

            *choice = (
                extra_cost_per_machine * (month_yield.saturating_sub(base_month_yield) as u64),
                month_yield,
            );
        }

        // Fill the cache.

        for (month, (&demand, &total_demand)) in demands[0..n - 1].iter().zip(total_demands.iter()).enumerate().rev() {
            let max_stock = if month == 0 { 0 } else { total_demand };
            let next_month_start = cache_columns * (month + 1);
            let max_remain_stock = total_demand - demand;

            for stock in 0..=max_stock {
                let min_remain_stock = stock.saturating_sub(demand);

                let choice = cache[next_month_start..]
                    .iter()
                    .enumerate()
                    .take(max_remain_stock + 1)
                    .skip(min_remain_stock)
                    .map(|(remain_stock, (next_cost, _))| {
                        let month_yield = remain_stock + demand - stock;
                        let extra_cost = extra_cost_per_machine * month_yield.saturating_sub(base_month_yield) as u64;
                        let inventory_cost = holding_cost(remain_stock);

                        (next_cost + extra_cost + inventory_cost, month_yield)
                    })
                    .min_by_key(|(cost, _)| *cost)
                    .unwrap();

                cache[cache_columns * month + stock] = choice;
            }
        }

        // Build result.

        let mut stock = 0;

        for ((month_yield, month_cache), demand) in
            result.iter_mut().zip(cache.chunks_exact(cache_columns)).zip(demands)
        {
            *month_yield = month_cache[stock].1;

            stock += *month_yield;
            stock -= demand;
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plan_inventory() {
        let demands = [0, 5, 5, 0, 4, 0, 5, 0, 3, 3, 2, 2, 0, 5, 1, 4, 2, 2, 3, 4];
        let base_yield_per_month = 3;
        let extra_cost_per_machine = 2;
        let holding_cost = |i| i as _;

        let result = super::plan_inventory(&demands, base_yield_per_month, extra_cost_per_machine, holding_cost);
        let expected_result = [2, 3, 5, 1, 3, 2, 3, 0, 3, 3, 2, 2, 2, 3, 2, 3, 2, 2, 3, 4];

        assert_eq!(*result, expected_result);
    }
}
