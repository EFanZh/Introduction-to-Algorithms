use std::cmp::Reverse;
use std::iter;

pub fn longest_monotonically_increasing_subsequence<T: Ord + Clone>(s: &[T]) -> Box<[T]> {
    let invalid_index = s.len();
    let mut links = vec![invalid_index; s.len()];
    let mut cache = Vec::with_capacity(s.len());

    for (i, s_i) in s.iter().enumerate().rev() {
        if let Err(index) = cache.binary_search_by_key(&Reverse(s_i), |j| Reverse(&s[*j])) {
            if let Some(value) = cache.get_mut(index) {
                *value = i;
            } else {
                cache.push(i);
            }

            if let Some(&next) = cache.get(index.wrapping_sub(1)) {
                links[i] = next;
            }
        }
    }

    let mut head = cache.last().copied().unwrap_or(invalid_index);

    iter::from_fn(|| {
        s.get(head).map(|value| {
            head = links[head];

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
