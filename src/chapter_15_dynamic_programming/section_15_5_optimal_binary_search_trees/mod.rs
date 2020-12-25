use std::f64;

pub mod exercises;

// Optimal-BST(p, q, n)
//
//  1  let e[1‥n + 1, 0‥n], w[1‥n + 1, 0‥n],
//         and root[1‥n, 1‥n] be new tables
//  2  for i = 1 to n + 1
//  3      e[i, i - 1] = q_{i - 1}
//  4      w[i, i - 1] = q_{i - 1}
//  5  for l = 1 to n
//  6      for i = 1 to n - l + 1
//  7          j = i + l - 1
//  8          e[i, j] = ∞
//  9          w[i, j] = w[i, j - 1] + p_j + q_j
// 10          for r = i to j
// 11              t = e[i, r - 1] + e[r + 1, j] + w[i, j]
// 12              if t < e[i, j]
// 13                  e[i, j] = t
// 14                  root[i, j] = r
// 15  return e and root

#[allow(clippy::many_single_char_names)]
pub fn optimal_bst(p: &[f64], q: &[f64]) -> (Box<[f64]>, Box<[usize]>) {
    let n = p.len();
    let columns = q.len();
    let mut e = vec![0.0; columns * columns];
    let mut w = vec![0.0; columns * columns];
    let mut root = vec![0; n * n];

    for (i, q_i) in q.iter().enumerate() {
        e[columns * i + i] = *q_i;
        w[columns * i + i] = *q_i;
    }

    for l in 1..=n {
        for i in 0..=n - l {
            let j = i + l;
            let mut e_i_j = f64::INFINITY;
            let w_i_j = w[columns * i + (j - 1)] + p[j - 1] + q[j];
            let mut root_i_j = usize::MAX;

            for r in i..j {
                let temp = e[columns * i + r] + e[columns * (r + 1) + j] + w_i_j;

                if temp < e_i_j {
                    e_i_j = temp;
                    root_i_j = r;
                }
            }

            e[columns * i + j] = e_i_j;
            w[columns * i + j] = w_i_j;
            root[n * i + (j - 1)] = root_i_j;
        }
    }

    (e.into(), root.into())
}

#[cfg(test)]
mod tests {
    use super::optimal_bst;

    pub fn run_optimal_bst_test<F: FnMut(&[f64], &[f64]) -> (Box<[f64]>, Box<[usize]>)>(mut f: F) {
        let p = [0.15, 0.10, 0.05, 0.10, 0.20];
        let q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10];

        let (mut e, root) = f(&p, &q);

        e.iter_mut().for_each(|x| *x = (*x * 100.0).round() / 100.0);

        let expected_cost = &[
            0.05, 0.45, 0.90, 1.25, 1.75, 2.75, //
            0.00, 0.10, 0.40, 0.70, 1.20, 2.00, //
            0.00, 0.00, 0.05, 0.25, 0.60, 1.30, //
            0.00, 0.00, 0.00, 0.05, 0.30, 0.90, //
            0.00, 0.00, 0.00, 0.00, 0.05, 0.50, //
            0.00, 0.00, 0.00, 0.00, 0.00, 0.10, //
        ] as &[_];

        assert_eq!(*e, *expected_cost);

        assert_eq!(
            *root,
            [
                0, 0, 1, 1, 1, //
                0, 1, 1, 1, 3, //
                0, 0, 2, 3, 4, //
                0, 0, 0, 3, 4, //
                0, 0, 0, 0, 4, //
            ]
        );
    }

    #[test]
    fn test_optimal_bst() {
        run_optimal_bst_test(optimal_bst);
    }
}
