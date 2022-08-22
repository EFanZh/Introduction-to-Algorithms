use crate::utilities::Infinitable;
use num::Zero;
use std::collections::HashMap;
use std::ops::Add;

pub fn verify_shortest_paths_tree<W>(graph: &[&[(usize, W)]], source: usize, states: &[(Infinitable<W>, usize)]) -> bool
where
    W: Add<Output = W> + Clone + Ord + Zero,
{
    let n = graph.len();

    // Make sure the tree are is a subgraph of `graph`.

    if states.len() != n {
        return false; // Vertex count does not match.
    }

    if states[source].0 != Infinitable::Finite(W::zero()) {
        return false; // Source distance should be 0.
    }

    let mut tree = vec![Vec::new(); n];

    for (node, (_, parent)) in states.iter().cloned().enumerate() {
        if node == source {
            if parent != usize::MAX {
                return false; // The source does not have a parent.
            }
        } else if let Some(siblings) = tree.get_mut(parent) {
            siblings.push(node);
        } else if parent != usize::MAX {
            return false; // Nonexistent vertex detected.
        }
    }

    let mut neighbors_map = HashMap::new();

    for ((children, neighbors), (source_distance, _)) in tree.into_iter().zip(graph).zip(states) {
        neighbors_map.extend(neighbors.iter().cloned());

        for child in children {
            if let Some(weight) = neighbors_map.get(&child) {
                if source_distance.clone() + weight.clone() != states[child].0 {
                    return false; // Edge weight does not match.
                }
            } else {
                return false; // This edge does not exits.
            }
        }

        neighbors_map.clear();
    }

    // Make sure we can’t relax any more.

    for (node, neighbors) in graph.iter().copied().enumerate() {
        let node_distance = states[node].0.clone();

        for (neighbor, weight) in neighbors.iter().cloned() {
            if node_distance.clone() + weight < states[neighbor].0 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::utilities::Infinitable;

    #[test]
    fn test_verify_shortest_paths_tree() {
        #[allow(trivial_casts)]
        let test_cases = [
            (
                (
                    &[&[(1_usize, 1)] as &[_], &[]] as &[&[_]],
                    0,
                    &[(Infinitable::Finite(0), usize::MAX)] as &[_],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1)], &[]],
                    0,
                    &[(Infinitable::Finite(1), usize::MAX), (Infinitable::Finite(1), 0)],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1)], &[]],
                    0,
                    &[(Infinitable::Finite(0), 0), (Infinitable::Finite(1), 0)],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1)], &[]],
                    0,
                    &[(Infinitable::Finite(0), usize::MAX), (Infinitable::Finite(1), 5)],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1)], &[]],
                    0,
                    &[(Infinitable::Finite(0), usize::MAX), (Infinitable::Finite(2), 0)],
                ),
                false,
            ),
            (
                (
                    &[&[], &[]],
                    0,
                    &[(Infinitable::Finite(0), usize::MAX), (Infinitable::Infinity, 0)],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1), (2, 1)], &[(2, 1)], &[]],
                    0,
                    &[
                        (Infinitable::Finite(0), usize::MAX),
                        (Infinitable::Finite(1), 0),
                        (Infinitable::Finite(2), 1),
                    ],
                ),
                false,
            ),
            (
                (
                    &[&[(1, 1), (2, 1)], &[(2, 1)], &[]],
                    0,
                    &[
                        (Infinitable::Finite(0), usize::MAX),
                        (Infinitable::Finite(1), 0),
                        (Infinitable::Finite(1), 0),
                    ],
                ),
                true,
            ),
            (
                (
                    &[&[(1, 1)], &[], &[]],
                    0,
                    &[
                        (Infinitable::Finite(0), usize::MAX),
                        (Infinitable::Finite(1), 0),
                        (Infinitable::Infinity, usize::MAX),
                    ],
                ),
                true,
            ),
        ];

        for ((graph, source, states), expected) in test_cases {
            assert_eq!(super::verify_shortest_paths_tree(graph, source, states), expected);
        }
    }
}
