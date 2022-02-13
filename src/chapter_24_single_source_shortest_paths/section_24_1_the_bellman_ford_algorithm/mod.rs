use crate::utilities::Infinitable;
use num::Zero;

pub mod exercises;

pub const FIGURE_24_4: [&[(usize, i32)]; 5] = [
    &[(1, 6), (3, 7)],
    &[(2, 5), (3, 8), (4, -4)],
    &[(1, -2)],
    &[(2, -3), (4, 9)],
    &[(2, 7), (0, 2)],
];

// Bellman-Ford(G, w, s)
//
// 1  Initialize-Single-Source(G, s)
// 2  for i = 1 to |G.V| - 1
// 3      for each edge (u, v) ∈ G.E
// 4          Relax(u, v, w)
// 5  for each edge (u, v) ∈ G.E
// 6      if v.d > u.d + w(u, v)
// 7          return false
// 8  return true

pub fn bellman_ford<W>(g: &[&[(usize, W)]], s: usize) -> (Vec<(Infinitable<W>, usize)>, bool)
where
    W: Clone + Ord + Zero,
{
    let n = g.len();
    let mut states = super::initialize_single_source(n, s);

    for _ in 1..n {
        for (u, neighbors) in g.iter().copied().enumerate() {
            for (v, w) in neighbors {
                super::relax(&mut states, u, *v, w.clone());
            }
        }
    }

    for (u, neighbors) in g.iter().copied().enumerate() {
        for (v, w) in neighbors {
            if states[*v].0 > states[u].0.clone() + w.clone() {
                return (states, false);
            }
        }
    }

    (states, true)
}

#[cfg(test)]
mod tests {
    use crate::utilities::Infinitable;

    type Graph<'a> = &'a [&'a [(usize, i32)]];
    type States<'a> = &'a [(Infinitable<i32>, usize)];

    #[test]
    fn test_bellman_ford() {
        let test_cases: [((Graph, usize), (States, bool)); 2] = [
            (
                (&super::FIGURE_24_4, 0),
                (
                    &[
                        (Infinitable::Finite(0), usize::MAX),
                        (Infinitable::Finite(2), 2),
                        (Infinitable::Finite(4), 3),
                        (Infinitable::Finite(7), 0),
                        (Infinitable::Finite(-2), 1),
                    ],
                    true,
                ),
            ),
            (
                (&[&[(1, 1)], &[(2, 1), (3, 1)], &[(1, -2)], &[]], 0),
                (
                    &[
                        (Infinitable::Finite(0), usize::MAX),
                        (Infinitable::Finite(-2), 2),
                        (Infinitable::Finite(0), 1),
                        (Infinitable::Finite(0), 1),
                    ],
                    false,
                ),
            ),
        ];

        for ((g, s), expected) in test_cases {
            let result = super::bellman_ford(g, s);

            assert_eq!((result.0.as_slice(), result.1), expected);
        }
    }
}
