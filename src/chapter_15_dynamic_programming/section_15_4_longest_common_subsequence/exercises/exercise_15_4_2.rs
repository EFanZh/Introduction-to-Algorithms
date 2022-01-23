fn print_lcs_2_helper<T: Clone + Eq>(c: &[usize], columns: usize, x: &[T], y: &[T], result: &mut Vec<T>) {
    if let (Some((x_tail, x_elements)), Some((y_tail, y_elements))) = (x.split_last(), y.split_last()) {
        if x_tail == y_tail {
            print_lcs_2_helper(c, columns, x_elements, y_elements, result);

            result.push(x_tail.clone());
        } else if c[columns * x.len() + y.len()] == c[columns * x_elements.len() + y.len()] {
            print_lcs_2_helper(c, columns, x_elements, y, result);
        } else {
            print_lcs_2_helper(c, columns, x, y_elements, result);
        }
    }
}

pub fn print_lcs_2<T: Clone + Eq>(c: &[usize], x: &[T], y: &[T]) -> Box<[T]> {
    let mut result = Vec::new();

    print_lcs_2_helper(c, y.len() + 1, x, y, &mut result);

    result.into()
}

#[cfg(test)]
mod tests {
    use super::super::super::lcs_length;

    #[test]
    fn test_print_lcs_2() {
        let x = b"ABCBDAB";
        let y = b"BDCABA";

        let (c, _) = lcs_length(x, y);

        assert_eq!(*super::print_lcs_2(&c, x, y), *b"BCBA");
    }
}
