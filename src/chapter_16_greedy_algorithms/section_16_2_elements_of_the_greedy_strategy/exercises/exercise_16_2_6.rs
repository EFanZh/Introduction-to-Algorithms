use super::super::super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::extra;
use super::super::super::super::chapter_9_medians_and_order_statistics::section_9_3_selection_in_worst_case_linear_time;
use super::super::super::super::utilities::KeyValuePair;
use num_rational::Ratio;

pub struct Item {
    pub value: u64,
    pub weight: u64,
}

struct ProcessedItem {
    index: usize,
    weight: u64,
}

fn partition(items: &mut [&KeyValuePair<Ratio<u64>, ProcessedItem>]) -> usize {
    let mut group_medians = items
        .chunks_mut(5)
        .map(|chunk| {
            chunk.sort();
            let middle = chunk.len() / 2;
            chunk[middle]
        })
        .collect::<Box<_>>();

    let num_groups = group_medians.len();

    let median_of_medians =
        *section_9_3_selection_in_worst_case_linear_time::select(&mut group_medians, 0, num_groups, num_groups / 2);

    let (left, _, _) = extra::partition_by_key(&mut group_medians, &median_of_medians);

    left.len()
}

fn quick_select(mut items: &mut [&KeyValuePair<Ratio<u64>, ProcessedItem>], mut capacity: u64) -> usize {
    let mut base = 0;

    while items.len() > 1 {
        let left_length = partition(items);
        let (left, middle_and_right) = items.split_at_mut(left_length);
        let (middle, right) = middle_and_right.split_first_mut().unwrap();
        let right_weight: u64 = right.iter().map(|item| item.value.weight).sum();

        if right_weight < capacity {
            let middle_and_right_weight = right_weight + middle.value.weight;

            if middle_and_right_weight >= capacity {
                return base + left_length;
            }

            items = left;
            capacity -= middle_and_right_weight;
        } else {
            base += left_length + 1;
            items = right;
        }
    }

    base
}

#[must_use]
pub fn select_items(items: &[Item], capacity: u64) -> Box<[u64]> {
    let processed_items = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            KeyValuePair::new(
                Ratio::new(item.value, item.weight),
                ProcessedItem {
                    index,
                    weight: item.weight,
                },
            )
        })
        .collect::<Box<_>>();

    let mut result = vec![0; items.len()];
    let mut processed_item_refs = processed_items.iter().collect::<Box<_>>();
    let left_size = quick_select(&mut processed_item_refs, capacity);
    let right = &processed_item_refs[left_size..];

    if let Some((first, rest)) = right.split_first() {
        let mut remaining_capacity = capacity;

        for KeyValuePair {
            value: ProcessedItem { index, .. },
            ..
        } in rest
        {
            let weight = items[*index].weight;

            result[*index] = weight;
            remaining_capacity -= weight;
        }

        result[first.value.index] = remaining_capacity.min(items[first.value.index].weight);
    }

    result.into()
}

#[cfg(test)]
mod tests {
    use super::{select_items, Item};

    #[test]
    fn test_select_items() {
        let test_cases = [
            ((&[] as &[_], 0), &[] as &[_]),
            ((&[], 4), &[]),
            ((&[(2, 3)], 0), &[0]),
            ((&[(2, 3)], 1), &[1]),
            ((&[(2, 3)], 3), &[3]),
            ((&[(2, 3)], 4), &[3]),
            ((&[(2, 3), (4, 3)], 2), &[0, 2]),
            ((&[(2, 3), (4, 3)], 4), &[1, 3]),
            ((&[(2, 3), (4, 3)], 8), &[3, 3]),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 0),
                &[0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 1),
                &[0, 0, 0, 0, 1, 0, 0, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 2),
                &[0, 0, 0, 0, 2, 0, 0, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 4),
                &[0, 0, 0, 1, 3, 0, 0, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 8),
                &[0, 4, 0, 1, 3, 0, 0, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 16),
                &[0, 6, 0, 1, 3, 0, 6, 0],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 32),
                &[2, 6, 0, 1, 3, 8, 9, 3],
            ),
            (
                (&[(8, 7), (7, 6), (2, 5), (2, 1), (8, 3), (2, 8), (5, 9), (1, 3)], 64),
                &[7, 6, 5, 1, 3, 8, 9, 3],
            ),
        ];

        for ((items, capacity), expected) in &test_cases {
            let items = items
                .iter()
                .map(|&(value, weight)| Item { value, weight })
                .collect::<Box<_>>();

            assert_eq!(*select_items(&items, *capacity), **expected);
        }
    }
}
