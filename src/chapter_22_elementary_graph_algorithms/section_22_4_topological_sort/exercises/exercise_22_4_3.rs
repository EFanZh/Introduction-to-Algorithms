fn helper(graph: &[Vec<usize>], parent: usize, node: usize, visited: &mut [bool]) -> bool {
    visited[node] = true;

    for &next in &graph[node] {
        if !visited[next] {
            if helper(graph, node, next, visited) {
                return true;
            }
        } else if next != parent {
            return true;
        }
    }

    false
}

#[must_use]
pub fn has_cycle(graph: &[Vec<usize>]) -> bool {
    let mut visited = vec![false; graph.len()];

    for node in 0..graph.len() {
        if !visited[node] && helper(graph, usize::MAX, node, &mut visited) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::has_cycle;

    #[test]
    fn test_has_cycle() {
        #[allow(trivial_casts)]
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
                true,
            ),
            (
                &[&[1, 2, 3], &[0, 4, 5], &[0], &[0, 6], &[1], &[1, 7], &[3], &[5]],
                false,
            ),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                has_cycle(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }
}
