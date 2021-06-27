use std::mem;

pub mod exercises;

/// Returns `k % m`.

#[must_use]
pub fn hash_by_division(k: u64, m: u64) -> u64 {
    k % m
}

/// Returns ⌊(2 ^ p) × ((k × (s / (2 ^ 64))) mod 1)⌋.

#[must_use]
pub fn hash_by_multiplication(k: u64, s: u64, p: usize) -> u64 {
    k.wrapping_mul(s) >> (mem::size_of::<u64>() * 8 - p)
}

#[cfg(test)]
mod tests {
    use super::{hash_by_division, hash_by_multiplication};

    #[test]
    fn test_hash_by_division() {
        assert_eq!(hash_by_division(7, 4), 3);
        assert_eq!(hash_by_division(7, 6), 1);
        assert_eq!(hash_by_division(7, 8), 7);
    }

    #[test]
    fn test_hash_by_multiplication() {
        let s = 11_400_714_819_323_198_485; // ⌊(sqrt(5) - 1) / 2 × (2 ^ 64)⌋.

        assert_eq!(hash_by_multiplication(5, s, 4), 1);
        assert_eq!(hash_by_multiplication(5, s, 6), 5);
        assert_eq!(hash_by_multiplication(5, s, 8), 23);
        assert_eq!(hash_by_multiplication(7, s, 4), 5);
        assert_eq!(hash_by_multiplication(7, s, 6), 20);
        assert_eq!(hash_by_multiplication(7, s, 8), 83);
    }
}
