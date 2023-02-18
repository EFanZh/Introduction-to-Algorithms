fn dfs(graph: &[Vec<usize>], from: usize, to: usize, cache: &mut Vec<Option<usize>>) -> usize {
    cache[from].unwrap_or_else(|| {
        let result = if from == to {
            1
        } else {
            graph[from].iter().map(|&next| dfs(graph, next, to, cache)).sum()
        };

        cache[from] = Some(result);

        result
    })
}

#[must_use]
pub fn get_number_of_paths(graph: &[Vec<usize>], s: usize, t: usize) -> usize {
    dfs(graph, s, t, &mut vec![None; graph.len()])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_number_of_paths() {
        #[allow(trivial_casts)] // Expected.
        let test_cases = [
            ((&[&[1_usize, 2] as &[usize], &[3], &[3], &[]] as &[&[_]], 0, 3), 2),
            ((&[&[4, 3, 1], &[3, 2, 4], &[3], &[4], &[]], 0, 3), 3),
            ((&[&[1], &[]], 0, 0), 1),
            ((&[&[1, 2, 3], &[2], &[3], &[]], 0, 3), 3),
            ((&[&[1, 3], &[2], &[3], &[]], 0, 3), 2),
        ];

        for ((graph, s, t), expected) in test_cases {
            assert_eq!(
                super::get_number_of_paths(
                    graph.iter().map(|nexts| nexts.to_vec()).collect::<Box<_>>().as_ref(),
                    s,
                    t
                ),
                expected
            );
        }
    }
}
