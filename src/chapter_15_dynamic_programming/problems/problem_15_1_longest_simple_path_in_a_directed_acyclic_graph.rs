use std::collections::HashMap;
use std::f64;

fn longest_simple_path_dfs<I: IntoIterator<Item = (f64, usize)>, F: FnMut(usize) -> I>(
    adj: &mut F,
    s: usize,
    t: usize,
    cache: &mut HashMap<usize, (f64, Option<usize>)>,
) -> f64 {
    #[allow(clippy::option_if_let_else)]
    if let Some(result) = cache.get(&s) {
        result.0
    } else {
        let result = if s == t {
            (0.0, None)
        } else {
            adj(s)
                .into_iter()
                .map(|(weight, next)| (weight + longest_simple_path_dfs(adj, next, t, cache), Some(next)))
                .max_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).unwrap())
                .unwrap_or((f64::NEG_INFINITY, None))
        };

        cache.insert(s, result);

        result.0
    }
}

pub fn longest_simple_path_dag<I: IntoIterator<Item = (f64, usize)>, F: FnMut(usize) -> I>(
    mut adj: F,
    mut s: usize,
    t: usize,
) -> Option<(f64, Box<[usize]>)> {
    let mut cache = HashMap::new();
    let max_cost = longest_simple_path_dfs(&mut adj, s, t, &mut cache);

    if max_cost.is_finite() {
        let mut path = vec![s];

        while let Some(next) = cache[&s].1 {
            path.push(next);
            s = next;
        }

        Some((max_cost, path.into()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::longest_simple_path_dag;

    #[test]
    fn test_longest_simple_path_dag() {
        let test_cases = vec![
            (vec![vec![]], 0, 0, Some((0.0, vec![0].into()))),
            (vec![vec![], vec![]], 0, 1, None),
            (vec![vec![(1.0, 1)], vec![]], 0, 1, Some((1.0, vec![0, 1].into()))),
            (
                vec![vec![(1.0, 1), (1.0, 2)], vec![(1.0, 2)], vec![]],
                0,
                2,
                Some((2.0, vec![0, 1, 2].into())),
            ),
            (
                vec![vec![(1.0, 1), (3.0, 2)], vec![(1.0, 2)], vec![]],
                0,
                2,
                Some((3.0, vec![0, 2].into())),
            ),
        ];

        for (graph, source, target, expected) in test_cases {
            assert_eq!(
                longest_simple_path_dag(|v| graph[v].iter().copied(), source, target),
                expected
            );
        }
    }
}
