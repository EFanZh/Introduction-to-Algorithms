pub fn reorder_sets<T: Ord>(a: &mut [T], b: &mut [T]) {
    assert_eq!(a.len(), b.len());

    a.sort_unstable();
    b.sort_unstable();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reorder_sets() {
        // It is really not necessary to test this on.

        let mut a = [5, 7, 1, 8];
        let mut b = [5, 2, 3, 9];

        super::reorder_sets(&mut a, &mut b);

        assert_eq!(a, [1, 5, 7, 8]);
        assert_eq!(b, [2, 3, 5, 9]);
    }
}
