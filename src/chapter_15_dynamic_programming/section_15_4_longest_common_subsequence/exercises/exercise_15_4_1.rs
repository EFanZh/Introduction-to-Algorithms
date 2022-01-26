#[must_use]
pub fn solve() -> Box<[u8]> {
    let x = [1, 0, 0, 1, 0, 1, 0, 1];
    let y = [0, 1, 0, 1, 1, 0, 1, 1, 0];

    let (_, b) = super::super::lcs_length(&x, &y);

    super::super::print_lcs(&b, &x, x.len(), y.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(*super::solve(), [1, 0, 0, 1, 1, 0]);
    }
}
