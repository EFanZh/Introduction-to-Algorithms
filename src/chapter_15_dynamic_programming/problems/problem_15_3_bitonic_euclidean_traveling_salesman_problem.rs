#[derive(Clone)]
struct Choice {
    length: f64,
    next: usize,
}

fn distance(p_0: &(f64, f64), p_1: &(f64, f64)) -> f64 {
    let dx = p_1.0 - p_0.0;
    let dy = p_1.1 - p_0.1;

    (dx * dx + dy * dy).sqrt()
}

fn reverse_copy<T: Copy>(target: &mut [T], source: &[T]) {
    for (t, s) in target.iter_mut().zip(source.iter().rev()) {
        *t = *s
    }
}

fn construct_tour(indices: &[usize], cache: &[Choice]) -> Box<[usize]> {
    let n = indices.len();
    let mut result = vec![0; n].into_boxed_slice();
    let mut start = 1;
    let mut end = n;
    let mut i = 1;

    while i != n - 1 {
        // Invariant: end - start = n - i.

        let new_i = cache[i - 1].next;
        let new_start = start + (new_i - i);

        result[start..new_start].copy_from_slice(&indices[i..new_i]);

        start = new_start;
        i = new_i;

        // Again, but with direction reversed.

        if i == n - 1 {
            break;
        }

        let new_i = cache[i - 1].next;
        let new_end = end - (new_i - i);

        reverse_copy(&mut result[new_end..end], &indices[i..new_i]);

        end = new_end;
        i = new_i;
    }

    result[start] = indices[i];

    result
}

pub fn shortest_tour(points: &[(f64, f64)]) -> Box<[usize]> {
    let n = points.len();
    let mut indices = (0..n).collect::<Box<_>>();

    indices.sort_by(|lhs, rhs| points[*lhs].0.partial_cmp(&points[*rhs].0).unwrap());

    if n < 4 {
        indices
    } else {
        let horizontal_distances = (1..n).fold(vec![0.0], |mut v, i| {
            v.push(v.last().unwrap() + distance(&points[indices[i - 1]], &points[indices[i]]));

            v
        });

        // cache[i].length: The minimal length bitonic path form points[indices[i]] to points[indices[i + 1]] that
        //                  visits all points that are on the right side of points[indices[i]].
        //
        // cache[i].next:   points[indices[cache[i].next]] is the next point on the minimal length path.
        //
        // Actually, we don’t need the real value of the last segment since it will be included in all paths.

        let mut cache = vec![Choice { length: 0.0, next: 0 }; n - 1];

        for (i, index) in indices.iter().enumerate().take(n - 2).rev() {
            let p_i = &points[*index];
            let anchor_distance = horizontal_distances[i + 1];

            cache[i] = cache
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(|(j, x)| Choice {
                    length: distance(p_i, &points[indices[j + 1]])
                        + x.length
                        + (horizontal_distances[j] - anchor_distance),
                    next: j + 1,
                })
                .min_by(|lhs, rhs| lhs.length.partial_cmp(&rhs.length).unwrap())
                .unwrap();
        }

        construct_tour(&indices, &cache)
    }
}

#[cfg(test)]
mod tests {
    use super::{construct_tour, shortest_tour, Choice};
    use std::iter;

    #[test]
    fn test_construct_tour() {
        fn run_test(nexts: &[usize], expected: &[usize]) {
            let indices = (0..nexts.len() + 2).collect::<Box<_>>();

            let cache = nexts
                .iter()
                .copied()
                .chain(iter::once(0))
                .map(|next| Choice { length: 0.0, next })
                .collect::<Box<_>>();

            assert_eq!(*construct_tour(&indices, &cache), *expected);
        }

        run_test(&[], &[0, 1]);
        run_test(&[2], &[0, 1, 2]);
        run_test(&[2, 3], &[0, 1, 3, 2]);
        run_test(&[3, 3], &[0, 1, 2, 3]);
        run_test(&[2, 3, 4], &[0, 1, 3, 4, 2]);
        run_test(&[2, 4, 4], &[0, 1, 4, 3, 2]);
        run_test(&[3, 3, 4], &[0, 1, 2, 4, 3]);
        run_test(&[3, 4, 4], &[0, 1, 2, 4, 3]);
        run_test(&[4, 3, 4], &[0, 1, 2, 3, 4]);
        run_test(&[4, 4, 4], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_shortest_tour() {
        fn run_test(points: &[(f64, f64)], expected: &[usize]) {
            assert_eq!(*shortest_tour(points), *expected);
        }

        run_test(&[], &[]);

        run_test(&[(0.0, 0.0)], &[0]);

        run_test(&[(0.0, 0.0), (1.0, 2.0)], &[0, 1]);
        run_test(&[(1.0, 2.0), (0.0, 0.0)], &[1, 0]);

        run_test(&[(0.0, 0.0), (1.0, 2.0), (2.0, 3.0)], &[0, 1, 2]);
        run_test(&[(0.0, 0.0), (2.0, 3.0), (1.0, 2.0)], &[0, 2, 1]);
        run_test(&[(1.0, 2.0), (0.0, 0.0), (2.0, 3.0)], &[1, 0, 2]);
        run_test(&[(2.0, 3.0), (0.0, 0.0), (1.0, 2.0)], &[1, 2, 0]);
        run_test(&[(1.0, 2.0), (2.0, 3.0), (0.0, 0.0)], &[2, 0, 1]);
        run_test(&[(2.0, 3.0), (1.0, 2.0), (0.0, 0.0)], &[2, 1, 0]);

        // Test case from the book.

        run_test(
            &[
                (0.0, 0.0),
                (7.0, 1.0),
                (5.0, 2.0),
                (2.0, 3.0),
                (8.0, 4.0),
                (6.0, 5.0),
                (1.0, 6.0),
            ],
            &[0, 6, 5, 4, 1, 2, 3],
        );
    }
}
