fn dfs_visit(graph: &[Vec<usize>], node: usize, visited: &mut [bool], cc: usize, result: &mut [usize]) {
    result[node] = cc;

    for &next in &graph[node] {
        if let value @ false = &mut visited[next] {
            *value = true;

            dfs_visit(graph, next, visited, cc, result);
        }
    }
}

#[must_use]
pub fn get_connected_components(graph: &[Vec<usize>]) -> Vec<usize> {
    let mut cc = 0;
    let mut visited = vec![false; graph.len()];
    let mut result = vec![0; graph.len()];

    for node in 0..graph.len() {
        if let value @ false = &mut visited[node] {
            *value = true;

            dfs_visit(graph, node, &mut visited, cc, &mut result);

            cc += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_connected_components() {
        #[allow(trivial_casts)] // Expected.
        let test_cases = [
            (
                &[
                    &[1_usize, 3] as &[usize],
                    &[0, 3, 4],
                    &[4, 5],
                    &[0, 1, 4],
                    &[1, 2, 3],
                    &[2, 5],
                ] as &[&[usize]],
                &[0, 0, 0, 0, 0, 0],
            ),
            (&[&[], &[2], &[1], &[4, 5], &[3], &[3]], &[0, 1, 1, 2, 2, 2]),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                super::get_connected_components(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }
}
