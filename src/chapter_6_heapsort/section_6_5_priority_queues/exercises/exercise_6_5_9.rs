use super::super::super::super::utilities::KeyValuePair;
use super::super::extra::{MinPriorityQueue, VecMinPriorityQueue};
use std::iter;

pub fn merge_sorted_lists<T: Ord, I: IntoIterator<Item = T>, V: IntoIterator<Item = I>>(
    sorted_lists: V,
) -> impl Iterator<Item = T> {
    let mut q = VecMinPriorityQueue::new();

    for list in sorted_lists {
        let mut iterator = list.into_iter();

        if let Some(value) = iterator.next() {
            q.insert(KeyValuePair::new(value, iterator));
        }
    }

    iter::from_fn(move || {
        if q.empty() {
            None
        } else {
            let KeyValuePair {
                key: result,
                value: mut iterator,
            } = q.extract_min();

            if let Some(value) = iterator.next() {
                q.insert(KeyValuePair::new(value, iterator));
            }

            Some(result)
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge_sorted_lists() {
        fn run_single_test(sorted_lists: Vec<Vec<i32>>, expected: &[i32]) {
            assert_eq!(
                super::merge_sorted_lists(sorted_lists).collect::<Box<_>>().as_ref(),
                expected
            );
        }

        run_single_test(vec![], &[]);
        run_single_test(vec![vec![]], &[]);
        run_single_test(vec![vec![], vec![]], &[]);
        run_single_test(vec![vec![], vec![], vec![]], &[]);

        run_single_test(vec![vec![1]], &[1]);
        run_single_test(vec![vec![1, 2]], &[1, 2]);
        run_single_test(vec![vec![1, 2, 3]], &[1, 2, 3]);

        run_single_test(vec![vec![1], vec![1]], &[1, 1]);
        run_single_test(vec![vec![1], vec![2]], &[1, 2]);
        run_single_test(vec![vec![2], vec![1]], &[1, 2]);

        run_single_test(
            vec![vec![1, 2, 3], vec![7, 8, 9], vec![2, 8], vec![5, 12]],
            &[1, 2, 2, 3, 5, 7, 8, 8, 9, 12],
        );
    }
}
