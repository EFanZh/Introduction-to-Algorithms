use ndarray::Array2;
use std::fmt::{Display, Formatter, Result};

pub mod exercises;

// Matrix-Chain-Order(p)
//
//  1  n = p.length - 1
//  2  let m[1‥n, 1‥n] and s[1‥n - 1, 2‥n] be new tables
//  3  for i = 1 to n
//  4      m[i, i] = 0
//  5  for l = 2 to n // l is the chain length
//  6      for i = 1 to n - l + 1
//  7          j = i + l - 1
//  8          m[i, j] = ∞
//  9          for k = i to j - 1
// 10              q = m[i, k] + m[k + 1, j] + p_{i-1} p_k p_j
// 11              if q < m[i, j]
// 12                  m[i, j] = q
// 13                  s[i, j] = k
// 14  return m and s

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn matrix_chain_order(p: &[usize]) -> (Array2<usize>, Array2<usize>) {
    let n = p.len() - 1;
    let mut m = Array2::zeros((n, n));
    let mut s = Array2::zeros((n - 1, n - 1));

    for l in 2..=n {
        // l is the chain length

        for i in 0..=n - l {
            let j = i + l;
            let mut m_i_j = usize::MAX;
            let mut s_i_j = usize::MAX;

            for k in i + 1..j {
                let q = m[[i, k - 1]] + m[[k, j - 1]] + p[i] * p[k] * p[j];

                if q < m_i_j {
                    m_i_j = q;
                    s_i_j = k;
                }
            }

            m[[i, j - 1]] = m_i_j;
            s[[i, j - 2]] = s_i_j;
        }
    }

    (m, s)
}

#[derive(PartialEq, Eq)]
pub enum Parens {
    Single(usize),
    Double(Box<Self>, Box<Self>),
}

impl Display for Parens {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Single(i) => write!(f, "A_{}", i)?,
            Self::Double(lhs, rhs) => write!(f, "({} {})", lhs, rhs)?,
        }

        Ok(())
    }
}

// Print-Optimal-Parens(s, i, j)
//
// 1  if i == j
// 2      print “A_i”
// 3  else print “(”
// 4       Print-Optimal-Parens(s, i, s[i, j])
// 5       Print-Optimal-Parens(s, s[i, j] + 1, j)
// 6       print “)”

#[must_use]
pub fn print_optimal_parens(s: &Array2<usize>, i: usize, j: usize) -> Parens {
    if i + 1 == j {
        Parens::Single(i)
    } else {
        let k = s[[i, j - 2]];

        Parens::Double(
            Box::new(print_optimal_parens(s, i, k)),
            Box::new(print_optimal_parens(s, k, j)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{matrix_chain_order, print_optimal_parens};
    use ndarray::array;

    #[test]
    fn test_matrix_chain_order() {
        let (m, s) = matrix_chain_order(&[30, 35, 15, 5, 10, 20, 25]);

        assert_eq!(
            m,
            array![
                [0, 15750, 7875, 9375, 11875, 15125],
                [0, 0, 2625, 4375, 7125, 10500],
                [0, 0, 0, 750, 2500, 5375],
                [0, 0, 0, 0, 1000, 3500],
                [0, 0, 0, 0, 0, 5000],
                [0, 0, 0, 0, 0, 0]
            ]
        );

        assert_eq!(
            s,
            array![
                [1, 1, 3, 3, 3],
                [0, 2, 3, 3, 3],
                [0, 0, 3, 3, 3],
                [0, 0, 0, 4, 5],
                [0, 0, 0, 0, 5]
            ]
        )
    }

    #[test]
    fn test_print_optimal_parens() {
        let p = [30, 35, 15, 5, 10, 20, 25];
        let (_, s) = matrix_chain_order(&p);

        assert_eq!(
            print_optimal_parens(&s, 0, p.len() - 1).to_string(),
            "((A_0 (A_1 A_2)) ((A_3 A_4) A_5))"
        )
    }
}
