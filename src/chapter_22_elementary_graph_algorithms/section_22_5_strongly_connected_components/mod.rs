pub mod exercises;

fn dfs_1(graph: &[Vec<usize>], nodes: impl IntoIterator<Item = usize>, visited: &mut [bool], result: &mut Vec<usize>) {
    for node in nodes {
        if let visited_value @ false = &mut visited[node] {
            *visited_value = true;

            dfs_1(graph, graph[node].iter().copied(), visited, result);

            result.push(node);
        }
    }
}

fn dfs_2(graph: &[Vec<usize>], node: usize, visited: &mut [bool], result: &mut Vec<usize>) {
    result.push(node);

    for &next in &graph[node] {
        if let visited_value @ false = &mut visited[next] {
            *visited_value = true;

            dfs_2(graph, next, visited, result);
        }
    }
}

// Strongly-Connected-Components(G)
//
// 1  call DFS(G) to compute finishing times u.f for each vertex u
// 2  compute G^T
// 3  call DFS(G^T), but in the main loop of DFS, consider the vertices in order of decreasing u.f (as computed in line 1)
// 4  output the vertices of each tree in the depth-first forest formed in line 3 as a separate strongly connected component

#[must_use]
pub fn strongly_connected_components(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    // First DFS.

    let mut visited = vec![false; g.len()];
    let mut nodes = Vec::with_capacity(g.len());

    dfs_1(g, 0..g.len(), &mut visited, &mut nodes);

    // Transpose graph.

    let mut transposed_graph = vec![Vec::new(); g.len()];

    for (node, nexts) in g.iter().enumerate() {
        for &next in nexts {
            transposed_graph[next].push(node);
        }
    }

    // Second DFS.

    visited.fill(false);

    let mut result = Vec::new();

    for node in nodes.into_iter().rev() {
        if let visited_value @ false = &mut visited[node] {
            *visited_value = true;

            let mut component = Vec::new();

            dfs_2(&transposed_graph, node, &mut visited, &mut component);

            result.push(component);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_strongly_connected_components() {
        #[allow(trivial_casts)]
        let test_cases = [(
            &[
                &[1_usize] as &[usize],
                &[2, 4, 5],
                &[3, 6],
                &[2, 7],
                &[0, 5],
                &[6],
                &[5, 7],
                &[7],
            ] as &[&[_]],
            &[&[0_usize, 4, 1] as &[usize], &[2, 3], &[6, 5], &[7]],
        )];

        for (g, expected) in test_cases {
            assert_eq!(
                super::strongly_connected_components(g.iter().map(|nexts| nexts.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }
}
