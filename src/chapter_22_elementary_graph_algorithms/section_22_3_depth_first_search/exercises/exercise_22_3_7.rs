#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Gray,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Attribute {
    pub color: Color,
    pub predecessor: Option<usize>,
    pub discovery_time: usize,
    pub finishing_time: usize,
}

pub fn dfs(graph: &[Vec<usize>]) -> Vec<Attribute> {
    let mut stack = Vec::new();
    let mut attributes = vec![Attribute::default(); graph.len()];
    let mut time = 0;

    for mut node in 0..graph.len() {
        if attributes[node].color == Color::White {
            'visit: loop {
                let attribute = &mut attributes[node];

                time += 1;
                attribute.discovery_time = time;
                attribute.color = Color::Gray;

                let mut iter = graph[node].iter().copied();

                // Return address.

                loop {
                    while let Some(next) = iter.next() {
                        let next_attribute = &mut attributes[next];

                        if next_attribute.color == Color::White {
                            next_attribute.predecessor = Some(node);

                            stack.push((node, iter));
                            node = next;

                            continue 'visit;
                        }
                    }

                    let attribute = &mut attributes[node];

                    attribute.color = Color::Black;
                    time += 1;
                    attribute.finishing_time = time;

                    // Apply continuation.

                    if let Some((next_node, next_iter)) = stack.pop() {
                        node = next_node;
                        iter = next_iter;
                    } else {
                        break 'visit;
                    }
                }
            }
        }
    }

    attributes
}

#[cfg(test)]
mod tests {
    use super::{dfs, Attribute, Color};

    #[test]
    fn test_dfs() {
        let test_cases = [(
            &[&[1_usize, 3] as &[usize], &[4], &[4, 5], &[1], &[3], &[5]] as &[&[usize]],
            &[
                (None, 1, 8),
                (Some(0), 2, 7),
                (None, 9, 12),
                (Some(4), 4, 5),
                (Some(1), 3, 6),
                (Some(2), 10, 11),
            ],
        )];

        for (graph, expected) in test_cases.iter().copied() {
            let result = dfs(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref());

            assert!(result.iter().all(|attribute| attribute.color == Color::Black));

            assert_eq!(
                result
                    .iter()
                    .map(
                        |&Attribute {
                             predecessor,
                             discovery_time,
                             finishing_time,
                             ..
                         }| (predecessor, discovery_time, finishing_time)
                    )
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }
}
