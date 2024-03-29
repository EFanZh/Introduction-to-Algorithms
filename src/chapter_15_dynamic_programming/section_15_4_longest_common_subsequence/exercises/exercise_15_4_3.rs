fn memoized_lcs_length_helper<T: Eq>(x: &[T], y: &[T], cache: &mut [Option<usize>], columns: usize) -> usize {
    cache[columns * x.len() + y.len()].unwrap_or_else(|| {
        let result = if let (Some((x_tail, x_elements)), Some((y_tail, y_elements))) = (x.split_last(), y.split_last())
        {
            if x_tail == y_tail {
                memoized_lcs_length_helper(x_elements, y_elements, cache, columns) + 1
            } else {
                let result_up = memoized_lcs_length_helper(x, y_elements, cache, columns);
                let result_left = memoized_lcs_length_helper(x, y_elements, cache, columns);

                result_up.max(result_left)
            }
        } else {
            0
        };

        cache[columns * x.len() + y.len()] = Some(result);

        result
    })
}

pub fn memoized_lcs_length<T: Eq>(x: &[T], y: &[T]) -> usize {
    let columns = y.len() + 1;
    let mut cache = vec![None; columns * (x.len() + 1)];

    memoized_lcs_length_helper(x, y, &mut cache, columns)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_memoized_lcs_length() {
        assert_eq!(super::memoized_lcs_length(b"ABCBDAB", b"BDCABA"), 4);
    }
}
