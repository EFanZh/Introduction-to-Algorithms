#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Color {
    White,
    Gray,
    Black(usize),
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum EdgeType {
    Tree,
    Back,
    Forward,
    Cross,
}

fn helper(
    graph: &[Vec<usize>],
    root: usize,
    node: usize,
    colors: &mut [Color],
    result: &mut Vec<((usize, usize), EdgeType)>,
) {
    colors[node] = Color::Gray;

    for &next in &graph[node] {
        match colors[next] {
            Color::White => {
                result.push(((node, next), EdgeType::Tree));

                helper(graph, root, next, colors, result);
            }
            Color::Gray => result.push(((node, next), EdgeType::Back)),
            Color::Black(next_root) => result.push((
                (node, next),
                if next_root == root {
                    EdgeType::Forward
                } else {
                    EdgeType::Cross
                },
            )),
        }
    }

    colors[node] = Color::Black(root);
}

pub fn get_edge_types(graph: &[Vec<usize>]) -> Vec<((usize, usize), EdgeType)> {
    let mut result = Vec::new();
    let mut colors = vec![Color::White; graph.len()];

    for node in 0..graph.len() {
        if colors[node] == Color::White {
            helper(graph, node, node, &mut colors, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{get_edge_types, EdgeType};

    #[test]
    fn test_get_edge_types() {
        let test_cases = [(
            &[&[1_usize, 3] as &[usize], &[4], &[4, 5], &[1], &[3], &[5]] as &[&[usize]],
            &[
                ((0, 1), EdgeType::Tree),
                ((1, 4), EdgeType::Tree),
                ((4, 3), EdgeType::Tree),
                ((3, 1), EdgeType::Back),
                ((0, 3), EdgeType::Forward),
                ((2, 4), EdgeType::Cross),
                ((2, 5), EdgeType::Tree),
                ((5, 5), EdgeType::Back),
            ],
        )];

        for (graph, expected) in test_cases.iter().copied() {
            assert_eq!(
                get_edge_types(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref()),
                expected
            );
        }
    }
}
