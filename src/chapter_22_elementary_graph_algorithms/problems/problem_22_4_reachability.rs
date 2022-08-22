fn dfs(graph: &[Vec<usize>], node: usize, label: usize, visited: &mut [bool], result: &mut [usize]) {
    result[node] = label;

    for &next in &graph[node] {
        if let visited_value @ false = &mut visited[next] {
            *visited_value = true;

            dfs(graph, next, label, visited, result);
        }
    }
}

#[must_use]
pub fn reachability(graph: &[&[usize]]) -> Box<[usize]> {
    let n = graph.len();
    let mut transposed_graph = vec![Vec::new(); n];

    for (node, &nexts) in graph.iter().enumerate() {
        for &next in nexts {
            transposed_graph[next].push(node);
        }
    }

    let mut visited = vec![false; n];
    let mut result = vec![0; n];

    for node in 0..n {
        if let visited_value @ false = &mut visited[node] {
            *visited_value = true;

            dfs(&transposed_graph, node, node, &mut visited, &mut result);
        }
    }

    result.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eachability() {
        #[allow(trivial_casts)] // Expected.
        let test_cases = [(
            &[&[1_usize] as &[usize], &[2], &[3], &[4], &[3]] as &[&[usize]],
            &[0_usize, 1, 2, 3, 3] as &[usize],
        )];

        for (graph, expected) in test_cases {
            assert_eq!(super::reachability(graph).as_ref(), expected);
        }
    }
}
