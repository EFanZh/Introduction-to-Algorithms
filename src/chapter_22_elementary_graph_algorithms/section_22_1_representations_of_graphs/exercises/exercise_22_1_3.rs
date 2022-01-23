use ndarray::Array2;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

pub fn transpose_adjacency_list<T: Hash + Eq + Clone, S: BuildHasher + Clone>(
    graph: &HashMap<T, Vec<T>, S>,
) -> HashMap<T, Vec<T>, S> {
    let mut result = HashMap::with_hasher(graph.hasher().clone());

    for (node, nexts) in graph {
        for next in nexts {
            match result.entry(next.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![node.clone()]);
                }
                Entry::Occupied(entry) => entry.into_mut().push(node.clone()),
            }
        }
    }

    result
}

#[must_use]
pub fn transpose_adjacency_matrix<T: Clone>(matrix: &Array2<T>) -> Array2<T> {
    matrix.t().to_owned()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_transpose_adjacency_list() {
        #[allow(trivial_casts)]
        let test_cases = [
            (
                &[(2, &[3, 4] as &[_])] as &[(_, &[_])],
                &[(3, &[2] as &[_]), (4, &[2])] as &[(_, &[_])],
            ),
            (
                &[(2, &[3, 4]), (3, &[4, 7, 8])],
                &[(3, &[2]), (4, &[2, 3]), (7, &[3]), (8, &[3])],
            ),
        ];

        for (graph, expected) in test_cases {
            let mut result = super::transpose_adjacency_list(
                &graph
                    .iter()
                    .copied()
                    .map(|(k, v)| (k, v.to_vec()))
                    .collect::<HashMap<_, _>>(),
            );

            for value in result.values_mut() {
                value.sort_unstable();
            }

            assert_eq!(result, expected.iter().copied().map(|(k, v)| (k, v.to_vec())).collect());
        }
    }

    #[test]
    fn test_transpose_adjacency_matrix() {
        let test_cases = [
            (
                ndarray::arr2(&[[false, true, false], [false, false, false], [false, false, false]]),
                ndarray::arr2(&[[false, false, false], [true, false, false], [false, false, false]]),
            ),
            (
                ndarray::arr2(&[[false, true, true], [false, false, false], [false, false, false]]),
                ndarray::arr2(&[[false, false, false], [true, false, false], [true, false, false]]),
            ),
        ];

        for (matrix, expected) in test_cases.iter().cloned() {
            assert_eq!(super::transpose_adjacency_matrix(&matrix), expected);
        }
    }
}
