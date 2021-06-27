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
    use super::search;

    #[allow(trivial_casts)]
    #[test]
    fn test_search() {
        assert_eq!(search(&[] as &[i32; 0], &1), None);

        assert_eq!(search(&[0], &1), None);
        assert_eq!(search(&[1], &1), Some(0));

        assert_eq!(search(&([0, 0]), &1), None);
        assert_eq!(search(&([0, 1]), &1), Some(1));
        assert_eq!(search(&([1, 0]), &1), Some(0));
        assert_eq!(search(&([1, 1]), &1), Some(0));
    }
}
