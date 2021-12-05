use crate::utilities::KeyValuePair;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};

// I am not very satisfied with this algorithm.

pub fn schedule<R: Ord>(requests: &[R], cache_size: usize) -> Box<[Option<&R>]> {
    let mut memory_queues = BTreeMap::new();

    for (i, request) in requests.iter().enumerate() {
        match memory_queues.entry(request) {
            Entry::Vacant(entry) => {
                entry.insert((i, Vec::new()));
            }
            Entry::Occupied(mut entry) => entry.get_mut().1.push(i),
        }
    }

    let mut memory_queue_iters = memory_queues
        .into_iter()
        .map(|(key, (i, value))| (key, (i, value.into_iter())))
        .collect::<BTreeMap<_, _>>();

    let mut result = Vec::new();
    let mut cache_elements = BTreeSet::new();

    for request in requests {
        let mut queue_entry = match memory_queue_iters.entry(request) {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => unreachable!(),
        };

        if let Some(next) = queue_entry.get_mut().1.next() {
            queue_entry.get_mut().0 = next;
        } else {
            queue_entry.remove();
        }

        if cache_elements.insert(request) {
            // Cache miss.

            if cache_elements.len() > cache_size {
                // Cache full.

                let element_to_evict = cache_elements
                    .iter()
                    .map(|&r| KeyValuePair::new(memory_queue_iters.get(r).map_or(usize::MAX, |(i, _)| *i), r))
                    .max()
                    .unwrap();

                assert!(cache_elements.remove(element_to_evict.value));

                result.push(Some(element_to_evict.value));
            } else {
                // Cache not full.

                result.push(None);
            }
        } else {
            // Cache hit.

            result.push(None);
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    use super::schedule;

    #[test]
    fn test_schedule() {
        fn run_test(requests: &str, cache_size: usize, expected_result: &[Option<char>]) {
            let result = schedule(&requests.chars().collect::<Box<_>>(), cache_size)
                .into_vec()
                .into_iter()
                .map(Option::<&char>::copied)
                .collect::<Box<_>>();

            assert_eq!(*result, *expected_result);
        }

        run_test(
            "dbdbdacdbacb",
            1,
            &[
                None,
                Some('b'),
                None,
                Some('b'),
                None,
                Some('a'),
                Some('c'),
                None,
                Some('d'),
                Some('a'),
                Some('c'),
                None,
            ],
        );

        run_test(
            "dbdbdacdbacb",
            2,
            &[
                None,
                None,
                None,
                None,
                None,
                Some('a'),
                Some('c'),
                None,
                None,
                Some('d'),
                Some('c'),
                None,
            ],
        );

        run_test(
            "dbdbdacdbacb",
            3,
            &[
                None,
                None,
                None,
                None,
                None,
                None,
                Some('c'),
                None,
                None,
                None,
                Some('d'),
                None,
            ],
        );

        run_test(
            "dbdbdacdbacb",
            4,
            &[None, None, None, None, None, None, None, None, None, None, None, None],
        );
    }
}
