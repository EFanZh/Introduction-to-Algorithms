use super::super::{Attribute, Color};

#[must_use]
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
    use super::super::super::tests;

    #[test]
    fn test_dfs() {
        tests::run_dfs_test(super::dfs);
    }
}
