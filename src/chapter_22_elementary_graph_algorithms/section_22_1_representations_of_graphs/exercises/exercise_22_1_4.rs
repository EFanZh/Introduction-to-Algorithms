use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash};

pub fn normalize<T: Hash + Eq + Clone, S: BuildHasher + Clone>(graph: &HashMap<T, Vec<T>, S>) -> HashMap<T, Vec<T>, S> {
    let mut result = HashMap::with_hasher(graph.hasher().clone());
    let mut cache = HashSet::new();

    for (node, nexts) in graph {
        result.insert(
            node.clone(),
            nexts
                .iter()
                .filter_map(|next| {
                    if next != node && cache.insert(next) {
                        Some(next.clone())
                    } else {
                        None
                    }
                })
                .collect(),
        );

        cache.clear();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::normalize;
    use std::collections::HashMap;

    #[test]
    fn test_normalize() {
        #[allow(trivial_casts)]
        let test_cases = [(
            &[(2, &[3, 4, 3] as &[_]), (3, &[2, 2, 3]), (4, &[2])] as &[(_, &[_])],
            &[(2, &[3, 4] as &[_]), (3, &[2]), (4, &[2])] as &[(_, &[_])],
        )];

        for (graph, expected) in test_cases {
            let mut result = normalize(
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
}
