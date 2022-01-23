use std::mem;

pub fn lcs_length_1<T: Eq>(x: &[T], y: &[T]) -> usize {
    let (x, y) = if y.len() < x.len() { (y, x) } else { (x, y) };
    let mut cache = vec![0; (x.len() + 1) * 2];
    let (mut cache_1, mut cache_2) = cache.split_at_mut(x.len() + 1);

    for y_j in y {
        for (i, x_i) in x.iter().enumerate() {
            cache_2[i + 1] = if x_i == y_j {
                cache_1[i] + 1
            } else {
                cache_2[i].max(cache_1[i + 1])
            };
        }

        mem::swap(&mut cache_1, &mut cache_2);
    }

    *cache_1.last().unwrap()
}

pub fn lcs_length_2<T: Eq>(x: &[T], y: &[T]) -> usize {
    let (x, y) = if y.len() < x.len() { (y, x) } else { (x, y) };
    let mut cache = vec![0; x.len() + 1];

    for y_j in y {
        let mut old_value = 0;

        for (i, x_i) in x.iter().enumerate() {
            let new_value = if x_i == y_j {
                old_value + 1
            } else {
                cache[i].max(cache[i + 1])
            };

            old_value = mem::replace(&mut cache[i + 1], new_value);
        }
    }

    *cache.last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_memoized_lcs_length_1() {
        assert_eq!(super::lcs_length_1(b"ABCBDAB", b"BDCABA"), 4);
    }

    #[test]
    fn test_memoized_lcs_length_2() {
        assert_eq!(super::lcs_length_2(b"ABCBDAB", b"BDCABA"), 4);
    }
}
