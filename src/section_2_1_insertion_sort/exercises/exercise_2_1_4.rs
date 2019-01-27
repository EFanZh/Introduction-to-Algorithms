pub fn add_binary(a: &[u8], b: &[u8], c: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() + 1, c.len());

    let n = a.len();
    let mut carry = 0;

    for i in 0..n {
        let sum = a[n - 1 - i] + b[n - 1 - i] + carry;

        c[n - i] = sum % 2;
        carry = sum / 2;
    }

    c[0] = carry;
}

#[cfg(test)]
mod tests {
    use super::add_binary;

    fn run_add_binary_test<T: AsRef<[u8]>, U: AsRef<[u8]> + AsMut<[u8]> + Default>(a: T, b: T, expected: U) {
        let mut result: U = Default::default();

        add_binary(a.as_ref(), b.as_ref(), result.as_mut());

        assert_eq!(*result.as_ref(), *expected.as_ref());
    }

    #[test]
    fn test_add_binary() {
        run_add_binary_test([], [], [0]);

        run_add_binary_test([0], [0], [0, 0]);
        run_add_binary_test([0], [1], [0, 1]);
        run_add_binary_test([1], [0], [0, 1]);
        run_add_binary_test([1], [1], [1, 0]);

        run_add_binary_test([0, 0], [0, 0], [0, 0, 0]);
        run_add_binary_test([0, 0], [0, 1], [0, 0, 1]);
        run_add_binary_test([0, 0], [1, 0], [0, 1, 0]);
        run_add_binary_test([0, 0], [1, 1], [0, 1, 1]);
        run_add_binary_test([0, 1], [0, 0], [0, 0, 1]);
        run_add_binary_test([0, 1], [0, 1], [0, 1, 0]);
        run_add_binary_test([0, 1], [1, 0], [0, 1, 1]);
        run_add_binary_test([0, 1], [1, 1], [1, 0, 0]);
        run_add_binary_test([1, 0], [0, 0], [0, 1, 0]);
        run_add_binary_test([1, 0], [0, 1], [0, 1, 1]);
        run_add_binary_test([1, 0], [1, 0], [1, 0, 0]);
        run_add_binary_test([1, 0], [1, 1], [1, 0, 1]);
        run_add_binary_test([1, 1], [0, 0], [0, 1, 1]);
        run_add_binary_test([1, 1], [0, 1], [1, 0, 0]);
        run_add_binary_test([1, 1], [1, 0], [1, 0, 1]);
        run_add_binary_test([1, 1], [1, 1], [1, 1, 0]);
    }
}
