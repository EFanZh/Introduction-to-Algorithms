#[must_use]
pub fn parent(i: usize) -> usize {
    (i - 1) / 2
}

#[must_use]
pub fn left(i: usize) -> usize {
    i * 2 + 1
}

#[must_use]
pub fn right(i: usize) -> usize {
    i * 2 + 2
}

#[cfg(test)]
mod tests {
    //                          ┌───┐
    //                          │ 0 │
    //                          └─┬─┘
    //             ┌──────────────┴───────────────┐
    //           ┌─┴─┐                          ┌─┴─┐
    //           │ 1 │                          │ 2 │
    //           └─┬─┘                          └─┬─┘
    //      ┌──────┴──────┐               ┌───────┴───────┐
    //    ┌─┴─┐         ┌─┴─┐           ┌─┴─┐           ┌─┴─┐
    //    │ 3 │         │ 4 │           │ 5 │           │ 6 │
    //    └─┬─┘         └─┬─┘           └─┬─┘           └─┬─┘
    //   ┌──┴───┐      ┌──┴───┐       ┌───┴───┐       ┌───┴───┐
    // ┌─┴─┐  ┌─┴─┐  ┌─┴─┐  ┌─┴──┐  ┌─┴──┐  ┌─┴──┐  ┌─┴──┐  ┌─┴──┐
    // │ 7 │  │ 8 │  │ 9 │  │ 10 │  │ 11 │  │ 12 │  │ 13 │  │ 14 │
    // └───┘  └───┘  └───┘  └────┘  └────┘  └────┘  └────┘  └────┘

    #[test]
    fn test_parent() {
        assert_eq!(super::parent(1), 0);
        assert_eq!(super::parent(2), 0);
        assert_eq!(super::parent(3), 1);
        assert_eq!(super::parent(4), 1);
        assert_eq!(super::parent(5), 2);
        assert_eq!(super::parent(6), 2);
        assert_eq!(super::parent(7), 3);
        assert_eq!(super::parent(8), 3);
        assert_eq!(super::parent(9), 4);
        assert_eq!(super::parent(10), 4);
        assert_eq!(super::parent(11), 5);
        assert_eq!(super::parent(12), 5);
        assert_eq!(super::parent(13), 6);
        assert_eq!(super::parent(14), 6);
    }

    #[test]
    fn test_left() {
        assert_eq!(super::left(0), 1);
        assert_eq!(super::left(1), 3);
        assert_eq!(super::left(2), 5);
        assert_eq!(super::left(3), 7);
        assert_eq!(super::left(4), 9);
        assert_eq!(super::left(5), 11);
        assert_eq!(super::left(6), 13);
    }

    #[test]
    fn test_right() {
        assert_eq!(super::right(0), 2);
        assert_eq!(super::right(1), 4);
        assert_eq!(super::right(2), 6);
        assert_eq!(super::right(3), 8);
        assert_eq!(super::right(4), 10);
        assert_eq!(super::right(5), 12);
        assert_eq!(super::right(6), 14);
    }
}
