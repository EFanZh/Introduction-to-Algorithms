pub struct Item {
    pub value: u64,
    pub weight: usize,
}

#[must_use]
pub fn select_items(items: &[Item], capacity: usize) -> (u64, Box<[usize]>) {
    let cache_columns = capacity + 1;
    let mut cache = vec![(0, false); cache_columns * (items.len() + 1)];

    for (i, &Item { value, weight }) in items.iter().enumerate().rev() {
        let (this_row, next_row) = cache[cache_columns * i..].split_at_mut(cache_columns);

        for (j, cache_item) in this_row.iter_mut().enumerate() {
            let mut choice = (next_row[j].0, false);

            if let Some(remain_capacity) = j.checked_sub(weight) {
                let value_if_select = value + next_row[remain_capacity].0;

                if value_if_select > choice.0 {
                    choice = (value_if_select, true);
                }
            }

            *cache_item = choice;
        }
    }

    let mut c = capacity;

    (
        cache[capacity].0,
        items
            .iter()
            .zip(cache.chunks_exact(cache_columns))
            .enumerate()
            .filter_map(|(i, (Item { weight, .. }, cache_row))| {
                if cache_row[c].1 {
                    c -= weight;

                    Some(i)
                } else {
                    None
                }
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::{select_items, Item};

    #[test]
    fn test_select_items() {
        let items_data = [(60, 10), (100, 20), (120, 30)];
        let capacity = 50;

        let (total_value, selected_items) = select_items(
            &items_data
                .iter()
                .map(|&(value, weight)| Item { value, weight })
                .collect::<Box<_>>(),
            capacity,
        );

        assert_eq!(total_value, 220);
        assert_eq!(*selected_items, [1, 2]);
    }
}
