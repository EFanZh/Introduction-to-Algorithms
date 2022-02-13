fn dfs(graph: &[&[usize]], vertex: usize, cache: &mut [usize]) -> usize {
    let mut candidate = cache[vertex];

    if candidate == 0 {
        candidate = graph[vertex].iter().map(|&i| dfs(graph, i, cache)).sum::<usize>() + 1;
        cache[vertex] = candidate;
    }

    candidate
}

#[must_use]
pub fn critical_path(graph: &[&[usize]]) -> usize {
    let mut cache = vec![0; graph.len()];

    (0..graph.len()).map(|vertex| dfs(graph, vertex, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    type Graph<'a> = &'a [&'a [usize]];

    #[test]
    fn test_critical_path() {
        let test_cases: [(Graph, usize); 4] = [
            (&[], 0),
            (&[&[]], 1),
            (&[&[1], &[]], 3),
            (&[&[1], &[2, 3], &[], &[], &[1]], 13),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(super::critical_path(graph), expected);
        }
    }
}
