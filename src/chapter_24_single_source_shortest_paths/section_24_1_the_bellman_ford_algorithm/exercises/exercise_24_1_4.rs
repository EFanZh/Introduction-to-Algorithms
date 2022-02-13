use std::collections::VecDeque;

fn initialize_single_source(n: usize, s: usize) -> Vec<(i32, usize)> {
    let mut result = vec![(i32::MAX, usize::MAX); n];

    result[s].0 = 0;

    result
}

fn relax(states: &mut [(i32, usize)], source: usize, target: usize, weight: i32) {
    let candidate = states[source].0 + weight;
    let state = &mut states[target];

    if candidate < state.0 {
        state.0 = candidate;
        state.1 = source;
    }
}

fn bfs(graph: &[&[(usize, i32)]], mut source: usize, states: &mut [(i32, usize)], queue: &mut VecDeque<usize>) {
    loop {
        for &(target, _) in graph[source] {
            let state = &mut states[target];

            if state.0 != i32::MIN {
                state.0 = i32::MIN;
                state.1 = source;

                queue.push_back(target);
            }
        }

        if let Some(next_source) = queue.pop_front() {
            source = next_source;
        } else {
            break;
        }
    }
}

#[must_use]
pub fn bellman_ford(graph: &[&[(usize, i32)]], source: usize) -> Vec<(i32, usize)> {
    let n = graph.len();
    let mut states = initialize_single_source(n, source);

    for _ in 1..n {
        for (source, neighbors) in graph.iter().copied().enumerate() {
            for &(target, weight) in neighbors {
                relax(&mut states, source, target, weight);
            }
        }
    }

    let mut queue = VecDeque::new();

    for (source, neighbors) in graph.iter().copied().enumerate() {
        for &(target, weight) in neighbors {
            let candidate = states[source].0.saturating_add(weight);
            let distance = &mut states[target].0;

            if candidate < *distance {
                *distance = i32::MIN;

                bfs(graph, target, &mut states, &mut queue);
            }
        }
    }

    states
}

#[cfg(test)]
mod tests {
    type Graph<'a> = &'a [&'a [(usize, i32)]];
    type States<'a> = &'a [(i32, usize)];

    #[test]
    fn test_bellman_ford() {
        let test_cases: [((Graph, usize), States); 3] = [
            (
                (
                    &[
                        &[(1, 6), (3, 7)],
                        &[(2, 5), (3, 8), (4, -4)],
                        &[(1, -2)],
                        &[(2, -3), (4, 9)],
                        &[(2, 7), (0, 2)],
                    ],
                    0,
                ),
                &[(0, usize::MAX), (2, 2), (4, 3), (7, 0), (-2, 1)],
            ),
            (
                (&[&[(1, 1)], &[(2, 1), (3, 1)], &[(1, -2)], &[]], 0),
                &[(0, usize::MAX), (i32::MIN, 2), (i32::MIN, 1), (i32::MIN, 1)],
            ),
            (
                (&[&[(2, 1)], &[(0, -3)], &[(1, 1)]], 0),
                &[(i32::MIN, 1), (i32::MIN, 2), (i32::MIN, 0)],
            ),
        ];

        for ((graph, source), expected) in test_cases {
            assert_eq!(super::bellman_ford(graph, source), expected);
        }
    }
}
