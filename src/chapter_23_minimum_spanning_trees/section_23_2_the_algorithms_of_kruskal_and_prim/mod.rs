use std::cmp::Ordering;

fn find_set(data: &mut [(usize, usize)], key: usize) -> usize {
    let parent = data[key].0;

    if parent == usize::MAX {
        key
    } else {
        let root = find_set(data, parent);

        data[key].1 = root;

        root
    }
}

fn union(data: &mut [(usize, usize)], root_1: usize, root_2: usize) {
    match data[root_1].1.cmp(&data[root_2].1) {
        Ordering::Less => data[root_1].0 = root_2,
        Ordering::Equal => {
            data[root_1].1 += 1;
            data[root_2].0 = root_1;
        }
        Ordering::Greater => data[root_2].0 = root_1,
    }
}

// MST-Kruskal(G, w)
//
// 1  A = ∅
// 2  for each vertex v ∈ G.V
// 3      Make-Set(v)
// 4  sort the edges of G.E into nondecreasing order by weight w
// 5  for each edge (u, v) ∈ G.E, taken in nondecreasing order by weight
// 6      if Find-Set(u) ≠ Find-Set(v)
// 7          A = A ∪ {(u, v)}
// 8          Union(u, v)
// 9  return A

#[must_use]
pub fn mst_kruskal(g: &[&[(usize, usize)]]) -> Vec<(usize, usize)> {
    let mut union_find = vec![(usize::MAX, 0); g.len()];

    let mut edges = g
        .iter()
        .enumerate()
        .flat_map(|(from, nexts)| nexts.iter().map(move |&(to, weight)| ((from, to), weight)))
        .collect::<Vec<_>>();

    edges.sort_unstable_by_key(|&(_, weight)| weight);

    let mut result = Vec::with_capacity(g.len().saturating_sub(1));

    for ((from, to), _) in edges {
        let root_from = find_set(&mut union_find, from);
        let root_to = find_set(&mut union_find, to);

        if root_from != root_to {
            result.push((from, to));
            union(&mut union_find, root_from, root_to);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    type WeightedGraph<'a> = &'a [&'a [(usize, usize)]];
    type Edges<'a> = &'a [(usize, usize)];

    const TEST_CASES: [(WeightedGraph, Edges); 1] = [(
        &[
            &[(1, 4), (7, 8)],
            &[(0, 4), (2, 8), (7, 11)],
            &[(1, 8), (3, 7), (5, 4), (8, 2)],
            &[(2, 7), (4, 9), (5, 14)],
            &[(3, 9), (5, 10)],
            &[(2, 4), (3, 14), (4, 10), (6, 2)],
            &[(5, 2), (7, 1), (8, 6)],
            &[(0, 8), (1, 11), (6, 1), (8, 7)],
            &[(2, 2), (6, 6), (7, 7)],
        ],
        &[(0, 1), (0, 7), (2, 3), (2, 5), (2, 8), (3, 4), (5, 6), (6, 7)],
    )];

    #[track_caller]
    fn check_result(mut result: Vec<(usize, usize)>, expected: &[(usize, usize)]) {
        for edge in &mut result {
            if edge.1 < edge.0 {
                *edge = (edge.1, edge.0);
            }
        }

        result.sort_unstable();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mst_kruskal() {
        for (g, expected) in TEST_CASES {
            check_result(super::mst_kruskal(g), expected);
        }
    }
}
