pub mod exercises;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arrow {
    UpLeft,
    Up,
    Left,
}

// Lcs-Length(X, Y)
//
//  1  m = X.length
//  2  n = Y.length
//  3  let b[1‥m, 1‥n] and c[0‥m, 0‥n] be new tables
//  4  for i = 1 to m
//  5      c[i, 0] = 0
//  6  for j = 0 to n
//  7      c[0, j] = 0
//  8  for i = 1 to m
//  9      for j = 1 to n
// 10          if x_i == y_j
// 11              c[i, j] = c[i - 1, j - 1] + 1
// 12              b[i, j] = “↖”
// 13          elseif c[i - 1, j] ≥ c[i, j - 1]
// 14              c[i, j] = c[i - 1, j]
// 15              b[i, j] = “↑”
// 16          else c[i, j] = c[i, j - 1]
// 17              b[i, j] = “←”
// 18  return c and b

pub fn lcs_length<T: Eq>(x: &[T], y: &[T]) -> (Box<[usize]>, Box<[Arrow]>) {
    let m = x.len();
    let n = y.len();
    let mut b = vec![Arrow::UpLeft; m * n];
    let mut c = vec![0; (m + 1) * (n + 1)];

    for i in 0..m {
        for j in 0..n {
            if x[i] == y[j] {
                c[(n + 1) * (i + 1) + (j + 1)] = c[(n + 1) * i + j] + 1;
                b[n * i + j] = Arrow::UpLeft;
            } else {
                let upper = c[(n + 1) * i + (j + 1)];
                let left = c[(n + 1) * (i + 1) + j];

                if upper >= left {
                    c[(n + 1) * (i + 1) + (j + 1)] = upper;
                    b[n * i + j] = Arrow::Up;
                } else {
                    c[(n + 1) * (i + 1) + (j + 1)] = left;
                    b[n * i + j] = Arrow::Left;
                }
            }
        }
    }

    (c.into(), b.into())
}

// Print-Lcs(b, X, i, j)
//
// 1  if i == 0 or j == 0
// 2      return
// 3  if b[i, j] == “↖”
// 4      Print-Lcs(b, X, i - 1, j - 1)
// 5      print x_i
// 6  elseif b[i, j] == “↑”
// 7      Print-Lcs(b, X, i - 1, j)
// 8  else Print-Lcs(b, X, i, j - 1)

fn print_lcs_helper<T: Clone>(b: &[Arrow], columns: usize, x: &[T], j: usize, result: &mut Vec<T>) {
    if let Some((tail, elements)) = x.split_last().filter(|_| j != 0) {
        match b[columns * elements.len() + (j - 1)] {
            Arrow::UpLeft => {
                print_lcs_helper(b, columns, elements, j - 1, result);
                result.push(tail.clone());
            }
            Arrow::Up => print_lcs_helper(b, columns, elements, j, result),
            Arrow::Left => print_lcs_helper(b, columns, x, j - 1, result),
        }
    }
}

pub fn print_lcs<T: Clone>(b: &[Arrow], x: &[T], i: usize, j: usize) -> Box<[T]> {
    let mut result = Vec::new();
    let columns = b.len() / x.len();

    print_lcs_helper(b, columns, &x[..i], j, &mut result);

    result.into()
}

#[cfg(test)]
mod tests {
    use super::{lcs_length, print_lcs, Arrow};

    #[test]
    fn test_lcs_length() {
        use Arrow::{Left, Up, UpLeft};

        let (c, b) = lcs_length(
            &"ABCBDAB".chars().collect::<Box<_>>(),
            &"BDCABA".chars().collect::<Box<_>>(),
        );

        let expected_c: &[usize] = &[
            0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 1, 1, 1, //
            0, 1, 1, 1, 1, 2, 2, //
            0, 1, 1, 2, 2, 2, 2, //
            0, 1, 1, 2, 2, 3, 3, //
            0, 1, 2, 2, 2, 3, 3, //
            0, 1, 2, 2, 3, 3, 4, //
            0, 1, 2, 2, 3, 4, 4, //
        ];

        assert_eq!(&*c, expected_c);

        let expected_b: &[Arrow] = &[
            Up, Up, Up, UpLeft, Left, UpLeft, //
            UpLeft, Left, Left, Up, UpLeft, Left, //
            Up, Up, UpLeft, Left, Up, Up, //
            UpLeft, Up, Up, Up, UpLeft, Left, //
            Up, UpLeft, Up, Up, Up, Up, //
            Up, Up, Up, UpLeft, Up, UpLeft, //
            UpLeft, Up, Up, Up, UpLeft, Up, //
        ];

        assert_eq!(&*b, expected_b);
    }

    #[test]
    fn test_print_lcs() {
        let x = "ABCBDAB".chars().collect::<Box<_>>();
        let y = "BDCABA".chars().collect::<Box<_>>();

        let (_, b) = lcs_length(&x, &y);

        assert_eq!(*print_lcs(&b, &x, x.len(), y.len()), ['B', 'C', 'B', 'A']);
    }
}
