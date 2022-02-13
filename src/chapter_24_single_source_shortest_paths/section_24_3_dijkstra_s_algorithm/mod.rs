use crate::utilities::Infinitable;
use num::Zero;
use std::mem;
use std::ops::Add;

pub const FIGURE_24_6: [&[(usize, u32)]; 5] = [
    &[(1, 10), (3, 5)],
    &[(2, 1), (3, 2)],
    &[(4, 4)],
    &[(1, 3), (2, 9), (4, 2)],
    &[(2, 6), (0, 7)],
];

fn build_heap(n: usize, source: usize) -> (Vec<usize>, Vec<usize>) {
    let mut heap = Vec::with_capacity(n);
    let mut heap_indices = Vec::with_capacity(n);

    if source == 0 {
        heap.extend(0..n);
        heap_indices.extend(0..n);
    } else {
        heap.push(source);
        heap.extend(0..source);
        heap.extend(source + 1..n);

        heap_indices.extend(1..=source);
        heap_indices.push(0);
        heap_indices.extend(source + 1..n);
    }

    (heap, heap_indices)
}

fn extract_min<W>(
    heap: &mut Vec<usize>,
    heap_indices: &mut [usize],
    states: &[(Infinitable<W>, usize)],
) -> Option<usize>
where
    W: Ord,
{
    heap.pop().map(|mut last| {
        if let Some(&first) = heap.first() {
            let vertex = mem::replace(&mut last, first);
            let distance = &states[vertex].0;
            let mut index = 0;

            loop {
                let left_index = index * 2 + 1;

                if let Some(&left) = heap.get(left_index) {
                    let left_distance = &states[left].0;
                    let right_index = left_index + 1;

                    if let Some(&right) = heap.get(right_index) {
                        let right_distance = &states[right].0;

                        let (child, child_distance, child_index) = if right_distance < left_distance {
                            (right, right_distance, right_index)
                        } else {
                            (left, left_distance, left_index)
                        };

                        if child_distance < distance {
                            heap[index] = child;
                            heap_indices[child] = mem::replace(&mut index, child_index);
                        } else {
                            break;
                        }
                    } else if left_distance < distance {
                        heap[index] = left;
                        heap_indices[left] = mem::replace(&mut index, left_index);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            heap[index] = vertex;
            heap_indices[vertex] = index;
        }

        last
    })
}

fn heap_decrease_key<W>(
    heap: &mut Vec<usize>,
    heap_indices: &mut [usize],
    vertex: usize,
    states: &[(Infinitable<W>, usize)],
) where
    W: Ord,
{
    let distance = &states[vertex].0;
    let mut index = heap_indices[vertex];

    loop {
        let parent_index = index.wrapping_sub(1) / 2;

        if let Some(&parent) = heap.get(parent_index) {
            if distance < &states[parent].0 {
                heap[index] = parent;
                heap_indices[parent] = mem::replace(&mut index, parent_index);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    heap[index] = vertex;
    heap_indices[vertex] = index;
}

fn relax<W>(
    states: &mut [(Infinitable<W>, usize)],
    source: usize,
    target: usize,
    weight: W,
    heap: &mut Vec<usize>,
    heap_indices: &mut [usize],
) where
    W: Add<Output = W> + Clone + Ord,
{
    let candidate = states[source].0.clone() + weight;
    let state = &mut states[target];

    if candidate < state.0 {
        state.0 = candidate;
        state.1 = source;

        heap_decrease_key(heap, heap_indices, target, states);
    }
}

// Dijkstra(G, w, s)
//
// 1  Initialize-Single-Source(G, s)
// 2  S = ∅
// 3  Q = G.V
// 4  while Q ≠ ∅
// 5      u = Extract-Min(Q)
// 6      S = S ∪ {u}
// 7      for each vertex v ∈ G.Adj[u]
// 8          Relax(u, v, w)

pub fn dijkstra<W>(g: &[&[(usize, W)]], s: usize) -> Vec<(Infinitable<W>, usize)>
where
    W: Add<Output = W> + Clone + Ord + Zero,
{
    let n = g.len();
    let mut states = super::initialize_single_source(n, s);
    let (mut q, mut q_indices) = build_heap(n, s);

    while let Some(u) = extract_min(&mut q, &mut q_indices, &states) {
        for (v, w) in g[u] {
            relax::<W>(&mut states, u, *v, w.clone(), &mut q, &mut q_indices);
        }
    }

    states
}

#[cfg(test)]
mod tests {
    use crate::utilities::Infinitable;

    type Graph<'a> = &'a [&'a [(usize, u32)]];
    type States<'a> = &'a [(Infinitable<u32>, usize)];

    #[test]
    fn test_dijkstra() {
        let test_cases: [((Graph, usize), States); 4] = [
            (
                (&super::FIGURE_24_6, 0),
                &[
                    (Infinitable::Finite(0), usize::MAX),
                    (Infinitable::Finite(8), 3),
                    (Infinitable::Finite(9), 1),
                    (Infinitable::Finite(5), 0),
                    (Infinitable::Finite(7), 3),
                ],
            ),
            (
                (&super::super::FIGURE_24_2, 0),
                &[
                    (Infinitable::Finite(0), usize::MAX),
                    (Infinitable::Finite(3), 0),
                    (Infinitable::Finite(9), 1),
                    (Infinitable::Finite(5), 0),
                    (Infinitable::Finite(11), 3),
                ],
            ),
            (
                (&super::super::FIGURE_24_2, 4),
                &[
                    (Infinitable::Finite(3), 4),
                    (Infinitable::Finite(6), 0),
                    (Infinitable::Finite(7), 4),
                    (Infinitable::Finite(8), 0),
                    (Infinitable::Finite(0), usize::MAX),
                ],
            ),
            (
                (
                    &[
                        &[(1, 3), (2, 1), (3, 4), (4, 1), (5, 5), (6, 9), (7, 2)],
                        &[],
                        &[],
                        &[],
                        &[],
                        &[],
                        &[],
                        &[],
                    ],
                    0,
                ),
                &[
                    (Infinitable::Finite(0), usize::MAX),
                    (Infinitable::Finite(3), 0),
                    (Infinitable::Finite(1), 0),
                    (Infinitable::Finite(4), 0),
                    (Infinitable::Finite(1), 0),
                    (Infinitable::Finite(5), 0),
                    (Infinitable::Finite(9), 0),
                    (Infinitable::Finite(2), 0),
                ],
            ),
        ];

        for ((g, s), expected) in test_cases {
            assert_eq!(super::dijkstra(g, s), expected);
        }
    }
}
