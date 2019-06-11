use std::mem::swap;

pub fn median_of_two_sorted_arrays<'a, T: Ord>(mut x: &'a [T], mut y: &'a [T]) -> &'a T {
    if y.len() < x.len() {
        swap(&mut x, &mut y);
    }

    let half_total_count = x.len() + (y.len() - x.len() + 1) / 2;
    let mut left = 0;
    let mut size = x.len();

    while size > 0 {
        let half_size = size / 2;
        let i = left + half_size;

        if y[half_total_count - i - 1] > x[i] {
            left = i + 1;
            size -= half_size + 1;
        } else {
            size = half_size;
        }
    }

    x.get(left.wrapping_sub(1))
        .max(y.get((half_total_count - left).wrapping_sub(1)))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::median_of_two_sorted_arrays;

    #[test]
    fn test_median_of_two_sorted_arrays() {
        let test_cases = vec![
            ((vec![], vec![0]), 0),
            ((vec![], vec![0, 1]), 0),
            ((vec![], vec![0, 1, 2]), 1),
            ((vec![], vec![0, 1, 2, 3]), 1),
            ((vec![], vec![0, 1, 2, 3, 4]), 2),
            ((vec![], vec![0, 1, 2, 3, 4, 5]), 2),
            ((vec![2], vec![1, 3]), 2),
            ((vec![1, 2], vec![3, 4]), 2),
            ((vec![1, 2], vec![-1, 3]), 1),
            ((vec![100_000], vec![100_001]), 100_000),
        ];

        for ((x, y), expected_median) in test_cases {
            assert_eq!(*median_of_two_sorted_arrays(&x, &y), expected_median);
            assert_eq!(*median_of_two_sorted_arrays(&y, &x), expected_median);
        }
    }
}
