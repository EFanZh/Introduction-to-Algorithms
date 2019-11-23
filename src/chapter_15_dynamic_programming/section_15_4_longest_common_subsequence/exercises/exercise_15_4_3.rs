fn memoized_lcs_length_helper<T: Eq>(x: &[T], y: &[T], cache: &mut [Option<usize>], columns: usize) -> usize {
    if let Some(result) = cache[columns * x.len() + y.len()] {
        result
    } else {
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
    }
}

pub fn memoized_lcs_length<T: Eq>(x: &[T], y: &[T]) -> usize {
    let columns = y.len() + 1;
    let mut cache = vec![None; columns * (x.len() + 1)];

    memoized_lcs_length_helper(x, y, &mut cache, columns)
}

#[cfg(test)]
mod tests {
    use super::memoized_lcs_length;

    #[test]
    fn test_memoized_lcs_length() {
        assert_eq!(memoized_lcs_length(b"ABCBDAB", b"BDCABA"), 4);
    }
}
