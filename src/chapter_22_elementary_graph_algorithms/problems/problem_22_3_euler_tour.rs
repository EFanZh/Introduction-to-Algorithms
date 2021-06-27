fn dfs(graph: &mut [Vec<usize>], node: usize, target: usize, result: &mut Vec<usize>) -> bool {
    let reachable = graph[node].pop().map_or_else(
        || node == target,
        |next| {
            dfs(graph, next, target, result) && graph[node].pop().map_or(true, |next| dfs(graph, next, node, result))
        },
    );

    if reachable {
        result.push(node);
    }

    reachable
}

// Not 100 % sure that this is correct.
#[must_use]
pub fn euler_tour(graph: &[&[usize]]) -> Option<Box<[usize]>> {
    let mut new_graph = Vec::new();
    let mut total_edges = 0;

    for nexts in graph {
        total_edges += nexts.len();
        new_graph.push(nexts.iter().rev().copied().collect::<Vec<_>>());
    }

    let mut result = Vec::new();

    if dfs(&mut new_graph, 0, 0, &mut result) {
        if result.len() == total_edges + 1 {
            result.reverse();

            Some(result.into_boxed_slice())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_euler_tour() {
        #[allow(trivial_casts)]
        let test_cases = [
            (
                &[&[1_usize] as &[usize], &[2], &[0]] as &[&[_]],
                Some(&[0_usize, 1, 2, 0] as &[_]),
            ),
            (
                &[&[1, 6], &[2], &[0, 3], &[4], &[2, 5], &[0], &[4]],
                Some(&[0, 1, 2, 0, 6, 4, 2, 3, 4, 5, 0]),
            ),
            (&[&[1], &[2, 3], &[0], &[4], &[1]], Some(&[0, 1, 3, 4, 1, 2, 0])),
            (&[&[1], &[2], &[3], &[4], &[]], None),
            (&[&[1, 2], &[0, 2], &[1]], None),
            (&[&[1], &[0], &[3], &[2]], None),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(super::euler_tour(graph).as_deref(), expected);
        }
    }
}
