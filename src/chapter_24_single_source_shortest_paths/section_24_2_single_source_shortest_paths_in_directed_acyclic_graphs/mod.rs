use crate::chapter_22_elementary_graph_algorithms::section_22_4_topological_sort;
use crate::utilities::Infinitable;
use num::Zero;

// Dag-Shortest-Paths(G, w, s)
//
// 1  topologically sort the vertices of G
// 2  Initialize-Single-Source(G, s)
// 3  for each vertex u, taken in topologically sorted order
// 4      for each vertex v ∈ G.Adj[u]
// 5          Relax(u, v, w)

pub fn dag_shortest_paths<W>(g: &[&[(usize, W)]], s: usize) -> Vec<(Infinitable<W>, usize)>
where
    W: Clone + Ord + Zero,
{
    let vertices = section_22_4_topological_sort::topological_sort(g);
    let mut states = super::initialize_single_source(g.len(), s);

    for u in vertices {
        for (v, w) in g[u] {
            super::relax(&mut states, u, *v, w.clone());
        }
    }

    states
}

#[cfg(test)]
mod tests {
    use crate::utilities::Infinitable;

    type Graph<'a> = &'a [&'a [(usize, i32)]];
    type States<'a> = &'a [(Infinitable<i32>, usize)];

    #[test]
    fn test_dag_shortest_paths() {
        let test_cases: [((Graph, usize), States); 1] = [(
            (
                &[
                    &[(1, 5), (2, 3)],
                    &[(2, 2), (3, 6)],
                    &[(3, 7), (4, 4), (5, 2)],
                    &[(4, -1), (5, 1)],
                    &[(5, -2)],
                    &[],
                ],
                1,
            ),
            &[
                (Infinitable::Infinity, usize::MAX),
                (Infinitable::Finite(0), usize::MAX),
                (Infinitable::Finite(2), 1),
                (Infinitable::Finite(6), 1),
                (Infinitable::Finite(5), 3),
                (Infinitable::Finite(3), 4),
            ],
        )];

        for ((g, s), expected) in test_cases {
            assert_eq!(super::dag_shortest_paths(g, s), expected);
        }
    }
}
