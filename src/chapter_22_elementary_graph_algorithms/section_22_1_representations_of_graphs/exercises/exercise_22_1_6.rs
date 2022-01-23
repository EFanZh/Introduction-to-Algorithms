use ndarray::{s, Array2, Axis};

#[must_use]
pub fn has_universal_sink(matrix: &Array2<bool>) -> bool {
    let n = matrix.nrows();
    let mut row = 0;
    let mut column = n - 1;

    while row != column {
        if matrix[(row, column)] {
            row += 1;
        } else {
            column -= 1;
        }
    }

    let row_view = matrix.index_axis(Axis(0), row);
    let column_view = matrix.index_axis(Axis(1), column);

    !row_view.iter().any(|x| *x)
        && column_view.slice(s![..column]).iter().all(|x| *x)
        && column_view.slice(s![column + 1..]).iter().all(|x| *x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_has_universal_sink() {
        let test_cases = [
            (
                ndarray::arr2(&[
                    [false, true, false, false],
                    [false, false, true, false],
                    [false, false, false, true],
                    [false, false, false, false],
                ]),
                false,
            ),
            (
                ndarray::arr2(&[
                    [false, true, false, false],
                    [false, false, true, false],
                    [false, true, false, true],
                    [false, true, false, false],
                ]),
                false,
            ),
        ];

        for (matrix, expected) in test_cases.iter().cloned() {
            assert_eq!(super::has_universal_sink(&matrix), expected);
        }
    }
}
