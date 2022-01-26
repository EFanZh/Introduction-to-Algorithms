#[must_use]
pub fn mst_prim(graph: &[&[usize]]) -> Vec<(usize, usize)> {
    let n = graph.len();
    let mut states = graph[0].iter().map(|&distance| Some((0, distance))).collect::<Vec<_>>();

    states[0] = None;

    let mut result = Vec::with_capacity(n - 1);

    for _ in 1..n {
        let (node, (parent, _)) = states
            .iter()
            .enumerate()
            .filter_map(|(node, state)| state.map(|state| (node, state)))
            .min_by_key(|&(_, (_, distance))| distance)
            .unwrap();

        result.push((parent, node));
        states[node] = None;

        for (neighbor, &distance) in graph[node].iter().enumerate() {
            if let Some((parent, current_distance)) = &mut states[neighbor] {
                if distance < *current_distance {
                    *parent = node;
                    *current_distance = distance;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::super::tests;

    #[test]
    fn test_mst_prim() {
        type Graph<'a> = &'a [&'a [usize]];

        const INF: usize = usize::MAX;

        let test_cases: [(Graph, &[(usize, usize)]); 1] = [(
            &[
                &[INF, 4, INF, INF, INF, INF, INF, 8, INF],
                &[4, INF, 8, INF, INF, INF, INF, 11, INF],
                &[INF, 8, INF, 7, INF, 4, INF, INF, 2],
                &[INF, INF, 7, INF, 9, 14, INF, INF, INF],
                &[INF, INF, INF, 9, INF, 10, INF, INF, INF],
                &[INF, INF, 4, 14, 10, INF, 2, INF, INF],
                &[INF, INF, INF, INF, INF, 2, INF, 1, 6],
                &[8, 11, INF, INF, INF, INF, 1, INF, 7],
                &[INF, INF, 2, INF, INF, INF, 6, 7, INF],
            ],
            &[(0, 1), (1, 2), (2, 3), (2, 5), (2, 8), (3, 4), (5, 6), (6, 7)],
        )];

        for (graph, expected) in test_cases {
            tests::check_result(super::mst_prim(graph), expected);
        }
    }
}
