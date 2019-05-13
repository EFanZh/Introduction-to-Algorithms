pub fn parent(i: usize) -> usize {
    (i - 1) / 2
}

pub fn left(i: usize) -> usize {
    i * 2 + 1
}

pub fn right(i: usize) -> usize {
    i * 2 + 2
}

#[cfg(test)]
mod tests {
    use super::{left, parent, right};

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
        assert_eq!(parent(1), 0);
        assert_eq!(parent(2), 0);
        assert_eq!(parent(3), 1);
        assert_eq!(parent(4), 1);
        assert_eq!(parent(5), 2);
        assert_eq!(parent(6), 2);
        assert_eq!(parent(7), 3);
        assert_eq!(parent(8), 3);
        assert_eq!(parent(9), 4);
        assert_eq!(parent(10), 4);
        assert_eq!(parent(11), 5);
        assert_eq!(parent(12), 5);
        assert_eq!(parent(13), 6);
        assert_eq!(parent(14), 6);
    }

    #[test]
    fn test_left() {
        assert_eq!(left(0), 1);
        assert_eq!(left(1), 3);
        assert_eq!(left(2), 5);
        assert_eq!(left(3), 7);
        assert_eq!(left(4), 9);
        assert_eq!(left(5), 11);
        assert_eq!(left(6), 13);
    }

    #[test]
    fn test_right() {
        assert_eq!(right(0), 2);
        assert_eq!(right(1), 4);
        assert_eq!(right(2), 6);
        assert_eq!(right(3), 8);
        assert_eq!(right(4), 10);
        assert_eq!(right(5), 12);
        assert_eq!(right(6), 14);
    }
}
