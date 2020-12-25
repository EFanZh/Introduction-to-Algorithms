use std::f64;

#[allow(clippy::many_single_char_names)]
pub fn optimal_bst_fast(p: &[f64], q: &[f64]) -> (Box<[f64]>, Box<[usize]>) {
    let n = p.len();
    let columns = q.len();
    let mut e = vec![0.0; columns * columns];
    let mut w = vec![0.0; columns * columns];
    let mut root = vec![0; n * n];

    for (i, q_i) in q.iter().enumerate() {
        e[columns * i + i] = *q_i;
        w[columns * i + i] = *q_i;
    }

    for i in 0..n {
        let w_i_j = w[columns * i + i] + p[i] + q[i + 1];

        e[columns * i + (i + 1)] = e[columns * i + i] + e[columns * (i + 1) + (i + 1)] + w_i_j;
        w[columns * i + (i + 1)] = w_i_j;
        root[n * i + i] = i;
    }

    for l in 2..=n {
        for i in 0..=n - l {
            let j = i + l;
            let mut e_i_j = f64::INFINITY;
            let w_i_j = w[columns * i + (j - 1)] + p[j - 1] + q[j];
            let mut root_i_j = usize::MAX;

            for r in root[n * i + (j - 2)]..=root[n * (i + 1) + (j - 1)] {
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
    use super::super::super::tests::run_optimal_bst_test;
    use super::optimal_bst_fast;

    #[test]
    fn test_optimal_bst_fast() {
        run_optimal_bst_test(optimal_bst_fast);
    }
}
