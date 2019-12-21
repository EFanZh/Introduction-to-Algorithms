use std::cmp::Ordering;

pub fn two_sum(s: &[i32], x: i32) -> bool {
    let a_sorted = {
        let mut a_temp = s.to_vec();
        a_temp.sort_unstable();
        a_temp
    };

    let mut range = a_sorted.as_slice();

    while range.len() > 1 {
        let first = range[0];
        let last = range[range.len() - 1];

        match (first + last).cmp(&x) {
            Ordering::Less => range = &range[1..],
            Ordering::Equal => return true,
            Ordering::Greater => range = &range[..range.len() - 1],
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[], 0), false);
        assert_eq!(two_sum(&[0], 0), false);
        assert_eq!(two_sum(&[0, 0], 0), true);
        assert_eq!(two_sum(&[0, 1], 0), false);
        assert_eq!(two_sum(&[1, 0], 1), true);
        assert_eq!(two_sum(&[0, 0, 0], 0), true);
        assert_eq!(two_sum(&[0, 0, 0], 1), false);
        assert_eq!(two_sum(&[0, 0, 1], 0), true);
        assert_eq!(two_sum(&[0, 0, 1], 1), true);
        assert_eq!(two_sum(&[0, 0, 1], 2), false);
        assert_eq!(two_sum(&[0, 1, 1], 0), false);
        assert_eq!(two_sum(&[0, 1, 1], 1), true);
        assert_eq!(two_sum(&[0, 1, 1], 2), true);
        assert_eq!(two_sum(&[0, 1, 1], 3), false);
        assert_eq!(two_sum(&[0, 1, 2], 0), false);
        assert_eq!(two_sum(&[0, 1, 2], 1), true);
        assert_eq!(two_sum(&[0, 1, 2], 2), true);
        assert_eq!(two_sum(&[0, 1, 2], 3), true);
        assert_eq!(two_sum(&[0, 1, 2], 4), false);
    }
}
