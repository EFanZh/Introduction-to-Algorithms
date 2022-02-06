fn relax(states: &mut [i32], from: usize, to: usize, weight: i32) {
    let new_distance = states[from] + weight;
    let distance = &mut states[to];

    *distance = (*distance).min(new_distance);
}

#[must_use]
pub fn delta_stars(graph: &[&[(usize, i32)]]) -> Vec<i32> {
    let n = graph.len();
    let mut states = vec![0; n];

    for _ in 1..n {
        for (from, neighbors) in graph.iter().copied().enumerate() {
            for &(to, weight) in neighbors {
                relax(&mut states, from, to, weight);
            }
        }
    }

    for (from, neighbors) in graph.iter().copied().enumerate() {
        for &(to, weight) in neighbors {
            let new_distance = states[from].saturating_add(weight);
            let distance = &mut states[to];

            if new_distance < *distance {
                *distance = i32::MIN;
            }
        }
    }

    states
}

#[cfg(test)]
mod tests {
    type Graph<'a> = &'a [&'a [(usize, i32)]];
    type States<'a> = &'a [i32];

    #[test]
    fn test_delta_stars() {
        let test_cases: [(Graph, States); 2] = [
            (
                &[
                    &[(1, 6), (3, 7)],
                    &[(2, 5), (3, 8), (4, -4)],
                    &[(1, -2)],
                    &[(2, -3), (4, 9)],
                    &[(2, 7), (0, 2)],
                ],
                &[-7, -5, -3, 0, -9],
            ),
            (
                &[&[(1, 1)], &[(2, 1), (3, 1)], &[(1, -2)], &[]],
                &[0, i32::MIN, i32::MIN, i32::MIN],
            ),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(super::delta_stars(graph), expected);
        }
    }
}
