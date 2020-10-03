use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

pub fn partition(graph: &[Vec<usize>]) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut groups = HashMap::new();
    let mut queue = VecDeque::new();

    for mut node in 0..graph.len() {
        if let Entry::Vacant(entry) = groups.entry(node) {
            entry.insert(false);

            loop {
                let group = groups[&node];

                for &next_node in &graph[node] {
                    match groups.entry(next_node) {
                        Entry::Occupied(entry) => {
                            if *entry.get() == group {
                                return None;
                            }
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(!group);
                            queue.push_back(next_node);
                        }
                    }
                }

                if let Some(next_i) = queue.pop_front() {
                    node = next_i;
                } else {
                    break;
                }
            }
        }
    }

    let mut group_1 = Vec::new();
    let mut group_2 = Vec::new();

    for node in 0..graph.len() {
        if groups[&node] {
            group_2.push(node);
        } else {
            group_1.push(node);
        }
    }

    Some((group_1, group_2))
}

#[cfg(test)]
mod tests {
    use super::partition;

    #[test]
    fn test_partition() {
        let test_cases = [
            (
                &[&[1_usize, 3] as &[usize], &[0, 2], &[1, 3], &[0, 2]] as &[&[usize]],
                Some((&[0_usize, 2] as &[usize], &[1_usize, 3] as &[usize])),
            ),
            (&[&[1, 2, 3], &[0, 2], &[0, 1, 3], &[0, 2]], None),
        ];

        for (graph, expected) in test_cases.iter().copied() {
            let result = partition(&graph.iter().map(|edge| edge.to_vec()).collect::<Box<_>>());

            assert_eq!(
                result.as_ref().map(|(left, right)| (left.as_slice(), right.as_slice())),
                expected
            );
        }
    }
}
