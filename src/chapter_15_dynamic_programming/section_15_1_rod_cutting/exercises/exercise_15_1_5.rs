#[must_use]
pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let c = a + b;

        a = b;
        b = c;
    }

    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(5), 5);
        assert_eq!(super::fibonacci(6), 8);
        assert_eq!(super::fibonacci(7), 13);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(9), 34);
        assert_eq!(super::fibonacci(10), 55);
    }
}
