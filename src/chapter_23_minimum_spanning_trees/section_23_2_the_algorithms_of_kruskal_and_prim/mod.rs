use crate::utilities::KeyValuePair;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::iter;

pub mod exercises;

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

// MST-Prim(G, w, r)
//
//  1  for each u ∈ G.V
//  2      u.key = ∞
//  3      u.π = nil
//  4  r.key = 0
//  5  Q = G.V
//  6  while Q ≠ ∅
//  7      u = Extract-Min(Q)
//  8      for each v ∈ G.Adj[u]
//  9          if v ∈ Q and w(u, v) < v.key
// 10              v.π = u
// 11              v.key = w(u, v)

#[must_use]
pub fn mst_prim(g: &[&[(usize, usize)]], r: usize) -> Vec<(usize, usize)> {
    struct NodeProperty {
        weight: usize,
        parent: usize,
    }

    let n = g.len();

    let mut properties = iter::repeat_with(|| NodeProperty {
        weight: usize::MAX,
        parent: usize::MAX,
    })
    .take(n)
    .collect::<Vec<_>>();

    properties[r].weight = 0;

    let mut node = r;
    let mut queue = BinaryHeap::new();

    'outer: loop {
        for &(next, weight) in g[node] {
            let next_property = &mut properties[next];

            if weight < next_property.weight {
                *next_property = NodeProperty { weight, parent: node };

                queue.push(KeyValuePair::new(Reverse(weight), next));
            }
        }

        loop {
            if let Some(next_item) = queue.pop() {
                let next_weight = &mut properties[next_item.value].weight;

                if next_item.key.0 == *next_weight {
                    *next_weight = 0;
                    node = next_item.value;

                    break;
                }
            } else {
                break 'outer;
            }
        }
    }

    properties
        .into_iter()
        .enumerate()
        .filter_map(|(i, NodeProperty { parent, .. })| (parent != usize::MAX).then_some((parent, i)))
        .collect()
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
    pub fn check_result(mut result: Vec<(usize, usize)>, expected: &[(usize, usize)]) {
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

    #[test]
    fn test_mst_prim() {
        for (g, expected) in TEST_CASES {
            check_result(super::mst_prim(g, 0), expected);
        }
    }
}
