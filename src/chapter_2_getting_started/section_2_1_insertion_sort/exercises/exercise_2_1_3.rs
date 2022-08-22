pub fn search<U, T: PartialEq<U>>(a: &[T], v: &U) -> Option<usize> {
    for (i, item) in a.iter().enumerate() {
        if *item == *v {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[allow(trivial_casts)] // Expected.
    #[test]
    fn test_search() {
        assert_eq!(super::search(&[] as &[i32; 0], &1), None);

        assert_eq!(super::search(&[0], &1), None);
        assert_eq!(super::search(&[1], &1), Some(0));

        assert_eq!(super::search(&([0, 0]), &1), None);
        assert_eq!(super::search(&([0, 1]), &1), Some(1));
        assert_eq!(super::search(&([1, 0]), &1), Some(0));
        assert_eq!(super::search(&([1, 1]), &1), Some(0));
    }
}
