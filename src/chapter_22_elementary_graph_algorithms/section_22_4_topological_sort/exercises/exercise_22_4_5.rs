#[must_use]
pub fn topological_sort(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut in_degrees = vec![0; graph.len()];

    for nexts in graph.iter() {
        for &next in nexts {
            in_degrees[next] += 1;
        }
    }

    let mut queue = in_degrees
        .iter()
        .enumerate()
        .filter_map(|(i, &indegree)| if indegree == 0 { Some(i) } else { None })
        .collect::<Vec<_>>();

    let mut result = Vec::new();

    while let Some(node) = queue.pop() {
        result.push(node);

        for &next in &graph[node] {
            let in_degree = &mut in_degrees[next];

            if *in_degree == 1 {
                queue.push(next);
            } else {
                *in_degree -= 1;
            }
        }
    }

    if result.len() == graph.len() {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::topological_sort;
    use std::collections::HashSet;

    #[test]
    fn test_has_cycle() {
        #[allow(trivial_casts)]
        let test_cases = [
            (&[&[1_usize] as &[usize], &[]] as &[&[usize]], true),
            (&[&[1, 2], &[], &[], &[1, 2]], true),
            (&[&[]], true),
            (&[&[1], &[0]], false),
            (&[&[1], &[0], &[1]], false),
        ];

        for (graph, can_finish) in test_cases {
            let graph = graph.iter().map(|nexts| nexts.to_vec()).collect::<Box<_>>();
            let result = topological_sort(&graph);

            if can_finish {
                let result = result.unwrap();

                assert_eq!(result.iter().copied().collect::<HashSet<_>>().len(), graph.len());

                let mut indices = vec![0; graph.len()];

                for (i, node) in result.into_iter().enumerate() {
                    indices[node] = i;
                }

                for (node, nexts) in graph.iter().enumerate() {
                    for &next in nexts {
                        assert!(indices[node] < indices[next]);
                    }
                }
            } else {
                assert!(result.is_none());
            }
        }
    }
}
