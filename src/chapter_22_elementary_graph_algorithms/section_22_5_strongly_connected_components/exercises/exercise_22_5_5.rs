use std::collections::HashSet;

fn dfs_1(graph: &[Vec<usize>], nodes: impl IntoIterator<Item = usize>, visited: &mut [bool], result: &mut Vec<usize>) {
    for node in nodes {
        if let visited_value @ false = &mut visited[node] {
            *visited_value = true;

            dfs_1(graph, graph[node].iter().copied(), visited, result);

            result.push(node);
        }
    }
}

fn dfs_2(
    graph: &[Vec<usize>],
    node: usize,
    current_component: usize,
    node_components: &mut [usize],
    current_component_group: &mut Vec<usize>,
    scc_edges: &mut HashSet<(usize, usize)>,
) {
    current_component_group.push(node);

    for &next in &graph[node] {
        match &mut node_components[next] {
            node_component @ &mut usize::MAX => {
                *node_component = current_component;

                dfs_2(
                    graph,
                    next,
                    current_component,
                    node_components,
                    current_component_group,
                    scc_edges,
                );
            }
            &mut node_component => {
                if node_component != current_component {
                    scc_edges.insert((node_component, current_component));
                }
            }
        }
    }
}

#[must_use]
pub fn get_scc_graph(g: &[Vec<usize>]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
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

    let mut node_components = vec![usize::MAX; g.len()];
    let mut component_groups = Vec::new();
    let mut scc_edges = HashSet::new();

    for node in nodes.into_iter().rev() {
        if let node_component @ &mut usize::MAX = &mut node_components[node] {
            *node_component = component_groups.len();

            let mut component = Vec::new();

            dfs_2(
                &transposed_graph,
                node,
                *node_component,
                &mut node_components,
                &mut component,
                &mut scc_edges,
            );

            component_groups.push(component);
        }
    }

    // Build SCC graph.

    let mut graph = vec![Vec::new(); component_groups.len()];

    for (from, to) in scc_edges {
        graph[from].push(to);
    }

    (component_groups, graph)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_scc_graph() {
        #[allow(trivial_casts)]
        let test_cases = &[(
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
            (
                &[&[0_usize, 4, 1] as &[usize], &[2, 3], &[6, 5], &[7]] as &[&[_]],
                &[&[1_usize, 2] as &[usize], &[2, 3], &[3], &[]] as &[&[_]],
            ),
        )];

        for (g, expected) in test_cases {
            let mut result = super::get_scc_graph(g.iter().map(|nexts| nexts.to_vec()).collect::<Box<_>>().as_ref());

            for nexts in &mut result.1 {
                nexts.sort_unstable();
            }

            assert_eq!(result.0, expected.0);
            assert_eq!(result.1, expected.1);
        }
    }
}
