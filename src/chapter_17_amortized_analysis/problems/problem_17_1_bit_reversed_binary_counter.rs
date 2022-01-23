// Question a.

#[must_use]
pub fn rev_k(value: u64, k: usize) -> u64 {
    let mut result = 0;

    for i in 0..k {
        result |= ((value >> i) & 1) << (k - 1 - i);
    }

    result
}

#[must_use]
pub fn bit_reversal_permutation(k: usize) -> Box<[u64]> {
    (0..(1 << k)).map(|i| rev_k(i, k)).collect()
}

// Question c.

#[must_use]
pub fn bit_reversed_increment(mut value: u64, k: usize) -> u64 {
    let mut probe = 1 << (k - 1);

    while value & probe != 0 {
        value ^= probe; // This is the same as `value &= !probe;`.

        probe >>= 1;
    }

    value ^= probe;

    value
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rev_k() {
        assert_eq!(super::rev_k(0, 4), 0);
        assert_eq!(super::rev_k(1, 4), 8);
        assert_eq!(super::rev_k(2, 4), 4);
        assert_eq!(super::rev_k(3, 4), 12);
        assert_eq!(super::rev_k(4, 4), 2);
        assert_eq!(super::rev_k(5, 4), 10);
        assert_eq!(super::rev_k(6, 4), 6);
        assert_eq!(super::rev_k(7, 4), 14);
        assert_eq!(super::rev_k(8, 4), 1);
        assert_eq!(super::rev_k(9, 4), 9);
        assert_eq!(super::rev_k(10, 4), 5);
        assert_eq!(super::rev_k(11, 4), 13);
        assert_eq!(super::rev_k(12, 4), 3);
        assert_eq!(super::rev_k(13, 4), 11);
        assert_eq!(super::rev_k(14, 4), 7);
        assert_eq!(super::rev_k(15, 4), 15);
    }

    #[test]
    fn test_bit_reversal_permutation() {
        let expected_result = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

        assert_eq!(*super::bit_reversal_permutation(4), expected_result);
    }

    #[test]
    fn test_bit_reversed_increment() {
        assert_eq!(super::bit_reversed_increment(0, 3), 4);
        assert_eq!(super::bit_reversed_increment(4, 3), 2);
        assert_eq!(super::bit_reversed_increment(2, 3), 6);
        assert_eq!(super::bit_reversed_increment(6, 3), 1);
        assert_eq!(super::bit_reversed_increment(1, 3), 5);
        assert_eq!(super::bit_reversed_increment(5, 3), 3);
        assert_eq!(super::bit_reversed_increment(3, 3), 7);

        assert_eq!(super::bit_reversed_increment(0, 4), 8);
        assert_eq!(super::bit_reversed_increment(8, 4), 4);
        assert_eq!(super::bit_reversed_increment(4, 4), 12);
        assert_eq!(super::bit_reversed_increment(12, 4), 2);
        assert_eq!(super::bit_reversed_increment(2, 4), 10);
        assert_eq!(super::bit_reversed_increment(10, 4), 6);
        assert_eq!(super::bit_reversed_increment(6, 4), 14);
        assert_eq!(super::bit_reversed_increment(14, 4), 1);
        assert_eq!(super::bit_reversed_increment(1, 4), 9);
        assert_eq!(super::bit_reversed_increment(9, 4), 5);
        assert_eq!(super::bit_reversed_increment(5, 4), 13);
        assert_eq!(super::bit_reversed_increment(13, 4), 3);
        assert_eq!(super::bit_reversed_increment(3, 4), 11);
        assert_eq!(super::bit_reversed_increment(11, 4), 7);
        assert_eq!(super::bit_reversed_increment(7, 4), 15);
    }
}
