use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash};

pub fn square_adjacency_list<T: Hash + Eq + Clone, S: BuildHasher + Clone>(
    graph: &HashMap<T, Vec<T>, S>,
) -> HashMap<T, Vec<T>, S> {
    let mut result = HashMap::with_hasher(graph.hasher().clone());
    let mut cache = HashSet::new();

    for (node, nexts) in graph {
        let mut new_nexts = Vec::new();

        for next in nexts {
            if cache.insert(next) {
                new_nexts.push(next.clone());
            }

            if let Some(next_nexts) = graph.get(next) {
                for next_next in next_nexts {
                    if cache.insert(next_next) {
                        new_nexts.push(next_next.clone());
                    }
                }
            }
        }

        result.insert(node.clone(), new_nexts);

        cache.clear();
    }

    result
}

#[must_use]
pub fn square_adjacency_matrix(matrix: &Array2<bool>) -> Array2<bool> {
    let mut result = matrix.clone();

    for (mut target_row, row) in result.genrows_mut().into_iter().zip(matrix.genrows()) {
        for (target, column) in target_row.iter_mut().zip(matrix.gencolumns()) {
            if !*target {
                for (lhs, rhs) in row.into_iter().zip(column) {
                    if *lhs && *rhs {
                        *target = true;

                        break;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{square_adjacency_list, square_adjacency_matrix};
    use std::collections::HashMap;

    #[test]
    fn test_square_adjacency_list() {
        let test_cases = [(
            &[(2, &[3, 4] as &[_]), (3, &[4, 7]), (4, &[8, 9]), (7, &[12])] as &[(_, &[_])],
            &[
                (2, &[3, 4, 7, 8, 9] as &[_]),
                (3, &[4, 7, 8, 9, 12]),
                (4, &[8, 9]),
                (7, &[12]),
            ] as &[(_, &[_])],
        )];

        for (graph, expected) in test_cases.iter().copied() {
            let mut result = square_adjacency_list(
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
    fn test_square_adjacency_matrix() {
        let test_cases = [(
            ndarray::arr2(&[
                [false, true, false, false],
                [false, false, true, false],
                [false, false, false, true],
                [false, false, false, false],
            ]),
            ndarray::arr2(&[
                [false, true, true, false],
                [false, false, true, true],
                [false, false, false, true],
                [false, false, false, false],
            ]),
        )];

        for (matrix, expected) in test_cases.iter().cloned() {
            assert_eq!(square_adjacency_matrix(&matrix), expected);
        }
    }
}
