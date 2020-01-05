// Recursive-Activity-Selector(s, f, k, n)
//
// 1  m = k + 1
// 2  while m ≤ n and s[m] < f[k] // find the first activity in S_k to finish
// 3      m = m + 1
// 4  if m ≤ n
// 5       return {a_m} ∪ Recursive-Activity-Selector(s, f, m, n)
// 6  else return ∅

pub fn recursive_activity_selector(s: &[u64], f: &[u64], k: usize) -> Vec<usize> {
    let last_finish_time = k.checked_sub(1).map(|i| f[i]).unwrap_or(0);

    if let Some(m) = s
        .iter()
        .enumerate()
        .skip(k)
        .find_map(|(i, s_i)| if *s_i < last_finish_time { None } else { Some(i) })
    {
        let mut result = recursive_activity_selector(s, f, m + 1);

        result.push(m);

        result
    } else {
        Vec::new()
    }
}

// Greedy-Activity-Selector(s, f)
//
// 1  n = s.length
// 2  A = {a_1}
// 3  k = 1
// 4  for m = 2 to n
// 5      if s[m] ≥ f[k]
// 6          A = A ∪ {a_m}
// 7          k = m
// 8  return A

pub fn greedy_activity_selector(s: &[u64], f: &[u64]) -> Box<[usize]> {
    let mut last_finish_time = 0;

    s.iter()
        .enumerate()
        .filter_map(|(i, s_i)| {
            if *s_i >= last_finish_time {
                last_finish_time = f[i];

                Some(i)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{greedy_activity_selector, recursive_activity_selector};

    #[test]
    fn test_recursive_activity_selector() {
        let s = [1, 3, 0, 5, 3, 5, 6, 8, 8, 2, 12];
        let f = [4, 5, 6, 7, 9, 9, 10, 11, 12, 14, 16];

        assert_eq!(recursive_activity_selector(&s, &f, 0), [10, 7, 3, 0]);
    }

    #[test]
    fn test_greedy_activity_selector() {
        let s = [1, 3, 0, 5, 3, 5, 6, 8, 8, 2, 12];
        let f = [4, 5, 6, 7, 9, 9, 10, 11, 12, 14, 16];

        assert_eq!(*greedy_activity_selector(&s, &f), [0, 3, 7, 10]);
    }
}
