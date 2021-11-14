use std::cmp::Reverse;
use std::iter;

pub fn longest_monotonically_increasing_subsequence<T: Ord + Clone>(s: &[T]) -> Box<[T]> {
    let invalid_index = s.len();
    let mut cache: Vec<(usize, usize)> = vec![(0, invalid_index); s.len()];
    let mut result_length = 0;
    let mut result_head = invalid_index;

    for (i, s_i) in s.iter().enumerate().rev() {
        let (mut max_length, max_next) = cache
            .iter()
            .enumerate()
            .skip(i)
            .filter_map(|(j, &(length, _))| (s[j] > *s_i).then(|| (length, j)))
            .min_by_key(|(length, _)| Reverse(*length))
            .unwrap_or((0, invalid_index));

        max_length += 1;

        cache[i] = (max_length, max_next);

        if max_length > result_length {
            result_length = max_length;
            result_head = i;
        }
    }

    iter::from_fn(|| {
        s.get(result_head).map(|value| {
            result_head = cache[result_head].1;

            value.clone()
        })
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::longest_monotonically_increasing_subsequence;

    #[test]
    fn test_longest_monotonically_increasing_subsequence() {
        let test_cases: Vec<(&[i32], &[i32])> = vec![(&[10, 9, 2, 5, 3, 7, 101, 18], &[2, 5, 7, 101])];

        for (nums, expected) in test_cases {
            assert_eq!(*longest_monotonically_increasing_subsequence(nums), *expected);
        }
    }
}
