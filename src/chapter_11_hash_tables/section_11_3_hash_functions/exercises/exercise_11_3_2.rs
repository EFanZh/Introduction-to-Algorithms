#[must_use]
pub fn hash_string(s: &[u8], m: u32) -> u32 {
    let mut result = 0_u64;
    let m = u64::from(m);

    for c in s.iter().map(|c| u64::from(*c)) {
        result = (result * 128 + c) % m;
    }

    result as _
}

#[cfg(test)]
mod tests {
    use super::hash_string;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string(&[], 1), 0);
        assert_eq!(hash_string(&[], 2), 0);
        assert_eq!(hash_string(&[3], 1), 0);
        assert_eq!(hash_string(&[3], 2), 1);
        assert_eq!(hash_string(&[3, 5, 7], 1), 0);
        assert_eq!(hash_string(&[3, 5, 7], 2), 1);
        assert_eq!(hash_string(&[3, 5, 7], 3), 2);
        assert_eq!(hash_string(&[3, 5, 7], 4), 3);
        assert_eq!(hash_string(&[3, 5, 7], 5), 4);
        assert_eq!(hash_string(&[3, 5, 7], 6), 5);
        assert_eq!(hash_string(&[3, 5, 7], 7), 1);
        assert_eq!(hash_string(&[3, 5, 7], 8), 7);
    }
}
