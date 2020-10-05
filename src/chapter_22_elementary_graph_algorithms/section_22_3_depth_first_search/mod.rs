pub mod exercises;

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

// DFS-Visit(G, u)
//
//  1  time = time + 1          // white vertex u has just been discovered
//  2  u.d = time
//  3  u.color = gray
//  4  for each v ∈ G.Adj[u]    // explore edge (u, v)
//  5      if v.color == white
//  6          v.π = u
//  7          DFS-Visit(G, v)
//  8  u.color = black          // blacken u; it is finished
//  9  time = time + 1
// 10  u.f = time

pub fn dfs_visit(g: &[Vec<usize>], u: usize, time: &mut usize, attributes: &mut [Attribute]) {
    let attribute = &mut attributes[u];

    *time += 1; // white vertex u has just been discovered
    attribute.discovery_time = *time;
    attribute.color = Color::Gray;

    for &v in &g[u] {
        // explore edge (u, v)

        let v_attribute = &mut attributes[v];

        if v_attribute.color == Color::White {
            v_attribute.predecessor = Some(u);

            dfs_visit(g, v, time, attributes);
        }
    }

    let attribute_2 = &mut attributes[u];

    attribute_2.color = Color::Black; // blacken u; it is finished
    *time += 1;
    attribute_2.finishing_time = *time;
}

// DFS(G)
//
// 1  for each vertex u ∈ G.V
// 2      u.color = white
// 3      u.π = nil
// 4  time = 0
// 5  for each vertex u ∈ G.V
// 6      if u.color == white
// 7          DFS-Visit(G, u)

pub fn dfs(g: &[Vec<usize>]) -> Vec<Attribute> {
    let mut attributes = vec![Attribute::default(); g.len()];
    let mut time = 0;

    for u in 0..g.len() {
        if attributes[u].color == Color::White {
            dfs_visit(g, u, &mut time, &mut attributes);
        }
    }

    attributes
}

#[cfg(test)]
mod tests {
    use super::{dfs, Attribute, Color};

    pub fn run_dfs_test(mut f: impl FnMut(&[Vec<usize>]) -> Vec<Attribute>) {
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
            let result = f(graph.iter().map(|node| node.to_vec()).collect::<Box<_>>().as_ref());

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

    #[test]
    fn test_dfs() {
        run_dfs_test(dfs);
    }
}
