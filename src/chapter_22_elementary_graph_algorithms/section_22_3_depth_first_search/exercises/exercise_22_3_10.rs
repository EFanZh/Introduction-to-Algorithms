use std::collections::HashSet;

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

fn helper_directed(
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

                helper_directed(graph, root, next, colors, result);
            }
            Color::Gray => result.push(((node, next), EdgeType::Back)),
            Color::Black(next_root) => {
                if next_root == root {
                    result.push(((node, next), EdgeType::Forward))
                } else {
                    result.push(((node, next), EdgeType::Cross))
                }
            }
        }
    }

    colors[node] = Color::Black(root);
}

pub fn get_edge_types_directed(graph: &[Vec<usize>]) -> Vec<((usize, usize), EdgeType)> {
    let mut result = Vec::new();
    let mut colors = vec![Color::White; graph.len()];

    for node in 0..graph.len() {
        if colors[node] == Color::White {
            helper_directed(graph, node, node, &mut colors, &mut result);
        }
    }

    result
}

fn normalize_edge(source: usize, target: usize) -> (usize, usize) {
    if source < target {
        (source, target)
    } else {
        (target, source)
    }
}

fn helper_undirected(
    graph: &[Vec<usize>],
    root: usize,
    node: usize,
    colors: &mut [bool],
    visited: &mut HashSet<(usize, usize)>,
    result: &mut Vec<((usize, usize), EdgeType)>,
) {
    colors[node] = true;

    for &next in &graph[node] {
        let edge = normalize_edge(node, next);

        if colors[next] {
            if visited.insert(edge) {
                result.push((edge, EdgeType::Back));
            }
        } else {
            visited.insert(edge);
            result.push((edge, EdgeType::Tree));

            helper_undirected(graph, root, next, colors, visited, result);
        }
    }
}

pub fn get_edge_types_undirected(graph: &[Vec<usize>]) -> Vec<((usize, usize), EdgeType)> {
    let mut result = Vec::new();
    let mut colors = vec![false; graph.len()];
    let mut visited = HashSet::new();

    for node in 0..graph.len() {
        if !colors[node] {
            helper_undirected(graph, node, node, &mut colors, &mut visited, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{get_edge_types_directed, get_edge_types_undirected, EdgeType};

    #[test]
    fn test_get_edge_types_directed() {
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
                ((0, 3), EdgeType::Back),
                ((3, 4), EdgeType::Tree),
                ((1, 4), EdgeType::Back),
                ((2, 4), EdgeType::Tree),
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
