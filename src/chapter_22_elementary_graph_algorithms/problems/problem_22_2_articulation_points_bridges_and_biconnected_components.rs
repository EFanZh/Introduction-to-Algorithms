use std::{iter, mem};

type Edge = (usize, usize);

#[must_use]
pub fn compute_low_values(graph: &[Vec<usize>]) -> Box<[usize]> {
    fn dfs(
        graph: &[Vec<usize>],
        vertex: usize,
        parent: usize,
        time: &mut usize,
        states: &mut [Option<usize>],
    ) -> usize {
        let discovery_time = mem::replace(time, *time + 1);
        let mut low = discovery_time;

        states[vertex] = Some(discovery_time);

        for &next in &graph[vertex] {
            if next != parent {
                low = low.min(states[next].unwrap_or_else(|| dfs(graph, next, vertex, time, states)));
            }
        }

        states[vertex] = Some(low);

        low
    }

    let mut states = iter::repeat_with(|| None).take(graph.len()).collect::<Box<_>>();

    dfs(graph, 0, 0, &mut 0, &mut states);

    states.into_vec().into_iter().map(Option::unwrap).collect()
}

#[must_use]
pub fn compute_articulation_points(graph: &[Vec<usize>]) -> Box<[usize]> {
    fn dfs(
        graph: &[Vec<usize>],
        vertex: usize,
        parent: usize,
        time: &mut usize,
        states: &mut [Option<usize>],
        result: &mut Vec<usize>,
    ) -> usize {
        let discovery_time = mem::replace(time, *time + 1);
        let mut low = discovery_time;
        let mut is_articulation_point = false;

        states[vertex] = Some(discovery_time);

        for &next in &graph[vertex] {
            if next != parent {
                match states[next] {
                    None => {
                        let next_low = dfs(graph, next, vertex, time, states, result);

                        if next_low >= discovery_time && !is_articulation_point {
                            is_articulation_point = true;

                            result.push(vertex);
                        }

                        low = low.min(next_low);
                    }
                    Some(d) => low = low.min(d),
                }
            }
        }

        states[vertex] = Some(low);

        low
    }

    let mut states = iter::repeat_with(|| None).take(graph.len()).collect::<Box<_>>();
    let mut result = Vec::new();

    let mut time = 1;
    let mut root_children = 0;

    states[0] = Some(0);

    for &next in &graph[0] {
        if states[next] == None {
            dfs(graph, next, 0, &mut time, &mut states, &mut result);

            root_children += 1;
        }
    }

    if root_children > 1 {
        result.push(0);
    }

    result.into_boxed_slice()
}

#[must_use]
pub fn compute_bridges(graph: &[Vec<usize>]) -> Box<[Edge]> {
    fn dfs(
        graph: &[Vec<usize>],
        vertex: usize,
        parent: usize,
        time: &mut usize,
        states: &mut [Option<usize>],
        result: &mut Vec<Edge>,
    ) -> usize {
        let discovery_time = mem::replace(time, *time + 1);
        let mut low = discovery_time;

        states[vertex] = Some(discovery_time);

        for &next in &graph[vertex] {
            if next != parent {
                match states[next] {
                    None => {
                        let next_low = dfs(graph, next, vertex, time, states, result);

                        if next_low > discovery_time {
                            result.push((vertex, next));
                        }

                        low = low.min(next_low);
                    }
                    Some(d) => low = low.min(d),
                }
            }
        }

        states[vertex] = Some(low);

        low
    }

    let mut result = Vec::new();

    dfs(
        &graph,
        0,
        0,
        &mut 0,
        &mut iter::repeat_with(|| None).take(graph.len()).collect::<Box<_>>(),
        &mut result,
    );

    result.into_boxed_slice()
}

#[must_use]
pub fn compute_biconnected_components(graph: &[Vec<usize>]) -> Box<[Box<[Edge]>]> {
    enum State {
        Unvisited,
        Visiting(usize),
        Visited,
    }

    fn dfs(
        graph: &[Vec<usize>],
        vertex: usize,
        parent: usize,
        time: &mut usize,
        states: &mut [State],
        component: &mut Vec<Edge>,
        result: &mut Vec<Box<[Edge]>>,
    ) -> usize {
        let discovery_time = mem::replace(time, *time + 1);
        let mut low = discovery_time;

        states[vertex] = State::Visiting(discovery_time);

        for &next in &graph[vertex] {
            if next != parent {
                match states[next] {
                    State::Unvisited => {
                        let saved_length = component.len();

                        component.push((vertex, next));

                        let next_low = dfs(graph, next, vertex, time, states, component, result);

                        if next_low >= discovery_time {
                            result.push(component.drain(saved_length..).collect());
                        }

                        low = low.min(next_low);
                    }
                    State::Visiting(d) => {
                        low = low.min(d);
                        component.push((vertex, next))
                    }
                    State::Visited => {}
                }
            }
        }

        states[vertex] = State::Visited;

        low
    }

    let mut result = Vec::new();

    dfs(
        &graph,
        0,
        0,
        &mut 0,
        &mut iter::repeat_with(|| State::Unvisited)
            .take(graph.len())
            .collect::<Box<_>>(),
        &mut Vec::new(),
        &mut result,
    );

    result.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    fn get_clrs_graph() -> Box<[Vec<usize>]> {
        //   1   5─6     11          19──────┐
        //  ╱│╲   ╲│    ╱  ╲         │╲      │
        // 0─┼─3───4───9    13────14─┼─16───20
        //  ╲│╱   ╱│    ╲  ╱        ╲│╱
        //   2   7─8     12    21    15
        //       │             │ ╲  ╱ ╲
        //       10            22─17   18

        let result = vec![
            vec![1, 2, 3],            // 0
            vec![0, 2, 3],            // 1
            vec![0, 1, 3],            // 2
            vec![0, 1, 2, 4],         // 3
            vec![3, 5, 6, 7, 8, 9],   // 4
            vec![4, 6],               // 5
            vec![4, 5],               // 6
            vec![4, 8, 10],           // 7
            vec![4, 7],               // 8
            vec![4, 11, 12],          // 9
            vec![7],                  // 10
            vec![9, 13],              // 11
            vec![9, 13],              // 12
            vec![11, 12, 14],         // 13
            vec![13, 15, 16],         // 14
            vec![14, 16, 17, 18, 19], // 15
            vec![14, 15, 19, 20],     // 16
            vec![15, 21, 22],         // 17
            vec![15],                 // 18
            vec![15, 16, 20],         // 19
            vec![16, 19],             // 20
            vec![17, 22],             // 21
            vec![17, 21],             // 22
        ]
        .into_boxed_slice();

        let mut set = HashSet::new();

        for (node, nexts) in result.iter().enumerate() {
            for &next in nexts {
                if !set.remove(&(next, node)) {
                    set.insert((node, next));
                }
            }
        }

        assert!(set.is_empty());

        result
    }

    #[test]
    fn test_compute_low_values() {
        assert_eq!(
            super::compute_low_values(&get_clrs_graph()).as_ref(),
            &[0, 0, 0, 0, 4, 4, 4, 4, 4, 10, 9, 10, 10, 10, 14, 14, 14, 19, 22, 15, 16, 19, 19]
        );
    }

    #[test]
    fn test_compute_articulation_points() {
        let mut result = super::compute_articulation_points(&get_clrs_graph());

        result.sort_unstable();

        assert_eq!(result.as_ref(), &[3, 4, 7, 9, 13, 14, 15, 17]);
    }

    #[test]
    fn test_compute_articulation_points_root() {
        let mut result = super::compute_articulation_points(&[vec![1, 2], vec![0], vec![0]]);

        result.sort_unstable();

        assert_eq!(result.as_ref(), &[0]);
    }

    #[test]
    fn test_compute_bridges() {
        let mut result = super::compute_bridges(&get_clrs_graph());

        result.sort_unstable();

        assert_eq!(
            result.as_ref(),
            &[(3, 4), (4, 9), (7, 10), (13, 14), (15, 17), (15, 18)]
        );
    }

    #[test]
    fn test_compute_biconnected_components() {
        let mut result = super::compute_biconnected_components(&get_clrs_graph());

        for component in result.iter_mut() {
            component.sort_unstable();
        }

        result.sort_unstable();

        let expected: &[&[(usize, usize)]] = &[
            &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 0), (3, 1)],
            &[(3, 4)],
            &[(4, 5), (5, 6), (6, 4)],
            &[(4, 7), (7, 8), (8, 4)],
            &[(4, 9)],
            &[(7, 10)],
            &[(9, 11), (11, 13), (12, 9), (13, 12)],
            &[(13, 14)],
            &[(14, 15), (15, 16), (16, 14), (16, 19), (19, 15), (19, 20), (20, 16)],
            &[(15, 17)],
            &[(15, 18)],
            &[(17, 21), (21, 22), (22, 17)],
        ];

        assert_eq!(result.len(), expected.len());

        for (component, &expected_component) in result.iter().zip(expected) {
            assert_eq!(component.as_ref(), expected_component);
        }
    }
}
