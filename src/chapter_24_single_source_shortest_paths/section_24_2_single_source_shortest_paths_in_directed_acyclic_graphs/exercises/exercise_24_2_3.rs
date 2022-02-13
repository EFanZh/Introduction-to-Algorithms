use crate::chapter_22_elementary_graph_algorithms::section_22_4_topological_sort;

fn relax(source: usize, candidate: i32, state: &mut (i32, usize)) {
    if candidate > state.0 {
        state.0 = candidate;
        state.1 = source;
    }
}

#[must_use]
pub fn critical_path(graph: &[(i32, &[usize])]) -> Vec<usize> {
    let vertices = section_22_4_topological_sort::topological_sort(graph);
    let mut states = vec![(0, usize::MAX); graph.len()];
    let mut final_state = (0, usize::MAX);

    for source in vertices {
        let (weight, neighbors) = graph[source];
        let candidate = states[source].0 + weight;

        for &target in neighbors {
            relax(source, candidate, &mut states[target]);
        }

        relax(source, candidate, &mut final_state);
    }

    let mut result = Vec::new();
    let mut vertex = final_state.1;

    while let Some(&(_, parent)) = states.get(vertex) {
        result.push(vertex);
        vertex = parent;
    }

    result.reverse();

    result
}

#[cfg(test)]
mod tests {
    type Graph<'a> = &'a [(i32, &'a [usize])];
    type Path<'a> = &'a [usize];

    #[test]
    fn test_critical_path() {
        let test_cases: [(Graph, Path); 4] = [
            (&[], &[]),
            (&[(1, &[])], &[0]),
            (&[(1, &[1]), (1, &[])], &[0, 1]),
            (&[(1, &[1]), (1, &[2, 3]), (1, &[]), (1, &[]), (2, &[1])], &[4, 1, 2]),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(super::critical_path(graph), expected);
        }
    }
}
