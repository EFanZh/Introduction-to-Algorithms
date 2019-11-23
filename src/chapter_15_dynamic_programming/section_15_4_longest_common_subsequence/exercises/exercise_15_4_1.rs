use super::super::{lcs_length, print_lcs};

pub fn solve() -> Box<[u8]> {
    let x = [1, 0, 0, 1, 0, 1, 0, 1];
    let y = [0, 1, 0, 1, 1, 0, 1, 1, 0];

    let (_, b) = lcs_length(&x, &y);

    print_lcs(&b, &x, x.len(), y.len())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(*solve(), [1, 0, 0, 1, 1, 0]);
    }
}
