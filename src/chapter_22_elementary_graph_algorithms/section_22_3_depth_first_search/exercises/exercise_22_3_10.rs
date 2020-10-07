#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Color {
    White,
    Gray,
    Black,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum EdgeType {
    Tree,
    Back,
    Forward,
    Cross,
}

#[derive(Copy, Clone)]
struct Attribute {
    color: Color,
    discovery_time: usize,
}

fn helper_directed(
    graph: &[Vec<usize>],
    node: usize,
    attributes: &mut [Attribute],
    time: &mut usize,
    result: &mut Vec<((usize, usize), EdgeType)>,
) {
    let attribute = &mut attributes[node];
    let discovery_time = *time;

    *time += 1;

    attribute.color = Color::Gray;
    attribute.discovery_time = discovery_time;

    for &next in &graph[node] {
        match attributes[next] {
            Attribute {
                color: Color::White, ..
            } => {
                result.push(((node, next), EdgeType::Tree));

                helper_directed(graph, next, attributes, time, result);
            }
            Attribute { color: Color::Gray, .. } => result.push(((node, next), EdgeType::Back)),
            Attribute {
                color: Color::Black,
                discovery_time: next_discovery_time,
            } => {
                if next_discovery_time < discovery_time {
                    result.push(((node, next), EdgeType::Cross))
                } else {
                    result.push(((node, next), EdgeType::Forward))
                }
            }
        }
    }

    attributes[node].color = Color::Black;
}

pub fn get_edge_types_directed(graph: &[Vec<usize>]) -> Vec<((usize, usize), EdgeType)> {
    let mut result = Vec::new();

    let mut attributes = vec![
        Attribute {
            color: Color::White,
            discovery_time: 0
        };
        graph.len()
    ];

    let mut time = 0;

    for node in 0..graph.len() {
        if attributes[node].color == Color::White {
            helper_directed(graph, node, &mut attributes, &mut time, &mut result);
        }
    }

    result
}

fn helper_undirected(
    graph: &[Vec<usize>],
    parent: usize,
    node: usize,
    colors: &mut [Color],
    result: &mut Vec<((usize, usize), EdgeType)>,
) {
    colors[node] = Color::Gray;

    for &next in &graph[node] {
        match colors[next] {
            Color::White => {
                result.push(((node, next), EdgeType::Tree));

                helper_undirected(graph, node, next, colors, result);
            }
            Color::Gray => {
                if next != parent {
                    result.push(((node, next), EdgeType::Back));
                }
            }
            Color::Black => {}
        }
    }

    colors[node] = Color::Black;
}

pub fn get_edge_types_undirected(graph: &[Vec<usize>]) -> Vec<((usize, usize), EdgeType)> {
    let mut result = Vec::new();
    let mut colors = vec![Color::White; graph.len()];

    for node in 0..graph.len() {
        if colors[node] == Color::White {
            helper_undirected(graph, usize::MAX, node, &mut colors, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{get_edge_types_directed, get_edge_types_undirected, EdgeType};

    #[test]
    fn test_get_edge_types_directed() {
        let test_cases = [
            (
                &[&[1_usize, 3] as &[usize], &[4], &[4, 5], &[1], &[3], &[5]] as &[&[usize]],
                &[
                    ((0_usize, 1_usize), EdgeType::Tree),
                    ((1, 4), EdgeType::Tree),
                    ((4, 3), EdgeType::Tree),
                    ((3, 1), EdgeType::Back),
                    ((0, 3), EdgeType::Forward),
                    ((2, 4), EdgeType::Cross),
                    ((2, 5), EdgeType::Tree),
                    ((5, 5), EdgeType::Back),
                ] as &[_],
            ),
            (
                &[&[1, 2], &[], &[1]],
                &[
                    ((0, 1), EdgeType::Tree),
                    ((0, 2), EdgeType::Tree),
                    ((2, 1), EdgeType::Cross),
                ],
            ),
        ];

        for (graph, expected) in test_cases.iter().copied() {
            assert_eq!(
                get_edge_types_directed(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }

    #[test]
    fn test_get_edge_types_undirected() {
        let test_cases = [(
            &[
                &[1_usize, 3] as &[usize],
                &[0, 3, 4],
                &[4, 5],
                &[0, 1, 4],
                &[1, 2, 3],
                &[2, 5],
            ] as &[&[usize]],
            &[
                ((0, 1), EdgeType::Tree),
                ((1, 3), EdgeType::Tree),
                ((3, 0), EdgeType::Back),
                ((3, 4), EdgeType::Tree),
                ((4, 1), EdgeType::Back),
                ((4, 2), EdgeType::Tree),
                ((2, 5), EdgeType::Tree),
                ((5, 5), EdgeType::Back),
            ],
        )];

        for (graph, expected) in test_cases.iter().copied() {
            assert_eq!(
                get_edge_types_undirected(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }
}
