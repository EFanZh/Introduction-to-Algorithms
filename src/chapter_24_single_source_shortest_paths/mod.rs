use crate::utilities::Infinitable;
use num::Zero;
use std::ops::Add;

pub mod section_24_1_the_bellman_ford_algorithm;

// Initialize-Single-Source(G, s)
//
// 1  for each vertex v ∈ G.V
// 2      v.d = ∞
// 3      v.π = nil
// 4  s.d = 0

#[must_use]
pub fn initialize_single_source<W: Clone + Zero>(n: usize, s: usize) -> Vec<(Infinitable<W>, usize)> {
    let mut result = vec![(Infinitable::Infinity, usize::MAX); n];

    result[s].0 = Infinitable::Finite(W::zero());

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
    let v_state = &mut states[v];

    if v_state.0 > candidate {
        v_state.0 = candidate;
        v_state.1 = u;
    }
}
