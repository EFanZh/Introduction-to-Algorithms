use crate::utilities::Infinitable;
use num::Zero;
use std::iter;
use std::ops::Add;

pub mod section_24_1_the_bellman_ford_algorithm;
pub mod section_24_2_single_source_shortest_paths_in_directed_acyclic_graphs;
pub mod section_24_3_dijkstra_s_algorithm;

pub const FIGURE_24_2: [&[(usize, u32)]; 5] = [
    &[(1, 3), (3, 5)],
    &[(2, 6), (3, 2)],
    &[(4, 2)],
    &[(1, 1), (2, 4), (4, 6)],
    &[(0, 3), (2, 7)],
];

// Initialize-Single-Source(G, s)
//
// 1  for each vertex v ∈ G.V
// 2      v.d = ∞
// 3      v.π = nil
// 4  s.d = 0

#[must_use]
pub fn initialize_single_source<W: Zero>(n: usize, s: usize) -> Vec<(Infinitable<W>, usize)> {
    let mut result = Vec::with_capacity(n);

    result.extend(iter::repeat_with(|| (Infinitable::Infinity, usize::MAX)).take(s));
    result.push((Infinitable::Finite(W::zero()), usize::MAX));
    result.extend(iter::repeat_with(|| (Infinitable::Infinity, usize::MAX)).take(n - (s + 1)));

    result
}

// Relax(u, v, w)
//
// 1  if v.d > u.d + w(u, v)
// 2      v.d = u.d + w(u, v)
// 3      v.π = u

pub fn relax<W>(states: &mut [(Infinitable<W>, usize)], u: usize, v: usize, w: W)
where
    W: Add<Output = W> + Clone + Ord,
{
    let candidate = states[u].0.clone() + w;
    let state = &mut states[v];

    if state.0 > candidate {
        state.0 = candidate;
        state.1 = u;
    }
}
