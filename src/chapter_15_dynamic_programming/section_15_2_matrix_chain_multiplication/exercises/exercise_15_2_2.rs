use ndarray::{Array2, LinalgScalar};

#[must_use]
pub fn matrix_chain_multiply<T: Clone + LinalgScalar>(
    a: &[Array2<T>],
    s: &Array2<usize>,
    i: usize,
    j: usize,
) -> Array2<T> {
    if i + 1 == j {
        a[i].clone()
    } else {
        let split = s[[i, j - 2]];
        let lhs = matrix_chain_multiply(a, s, i, split);
        let rhs = matrix_chain_multiply(a, s, split, j);

        lhs.dot(&rhs)
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    #[test]
    fn test_matrix_chain_multiply() {
        let a = [
            ndarray::array![[1, 2, 3], [4, 5, 6]],
            ndarray::array![[1, 2], [3, 4], [5, 6]],
            ndarray::array![[1, 2, 3, 4], [5, 6, 7, 8]],
        ];

        let p = iter::once(a[0].dim().0)
            .chain(a.iter().map(|x| x.dim().1))
            .collect::<Box<_>>();

        let (_, s) = super::super::super::matrix_chain_order(&p);

        let expected_result = ndarray::array![[162, 212, 262, 312], [369, 482, 595, 708]];

        assert_eq!(
            a[1..].iter().fold(a[0].clone(), |lhs, rhs| lhs.dot(rhs)),
            expected_result
        );

        assert_eq!(super::matrix_chain_multiply(&a, &s, 0, a.len()), expected_result);
    }
}
