use super::{
    adjust_on_left_child, adjust_on_left_child_black_sibling, adjust_on_left_child_black_sibling_red_root, insert,
    Color, Node, RedBlackTreeMap, Tree,
};

fn red<K, V>(key: K, value: V, left: Tree<K, V>, right: Tree<K, V>) -> Tree<K, V> {
    Some(Box::new(Node {
        color: Color::Red,
        key,
        value,
        left,
        right,
    }))
}

fn red_leaf<K, V>(key: K, value: V) -> Tree<K, V> {
    Some(Box::new(Node {
        color: Color::Red,
        key,
        value,
        left: None,
        right: None,
    }))
}

fn black<K, V>(key: K, value: V, left: Tree<K, V>, right: Tree<K, V>) -> Tree<K, V> {
    Some(Box::new(Node {
        color: Color::Black,
        key,
        value,
        left,
        right,
    }))
}

fn black_leaf<K, V>(key: K, value: V) -> Tree<K, V> {
    Some(Box::new(Node {
        color: Color::Black,
        key,
        value,
        left: None,
        right: None,
    }))
}

fn run_insertion_test(
    mut tree: Tree<i32, i32>,
    key: i32,
    value: i32,
    exprected_tree: Tree<i32, i32>,
    expected_result: Option<i32>,
) {
    let result = insert(&mut tree, key, value);

    assert_eq!(tree, exprected_tree);
    assert_eq!(result, expected_result);
}

// Insertion tests.

#[test]
fn test_red_black_tree_insert_root() {
    run_insertion_test(None, 2, 7, black_leaf(2, 7), None);
}

#[test]
fn test_red_black_tree_insert_root_equal() {
    run_insertion_test(black_leaf(2, 7), 2, 9, black_leaf(2, 9), Some(7));
}

#[test]
fn test_red_black_tree_insert_root_left_side_case_1() {
    run_insertion_test(
        black(4, 3, red_leaf(2, 7), red_leaf(7, 9)),
        1,
        5,
        black(4, 3, black(2, 7, red_leaf(1, 5), None), black_leaf(7, 9)),
        None,
    );

    run_insertion_test(
        black(4, 3, red_leaf(2, 7), red_leaf(7, 9)),
        3,
        5,
        black(4, 3, black(2, 7, None, red_leaf(3, 5)), black_leaf(7, 9)),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_left_side_case_2_and_3() {
    run_insertion_test(
        black(4, 3, red_leaf(2, 7), None),
        1,
        5,
        black(2, 7, red_leaf(1, 5), red_leaf(4, 3)),
        None,
    );

    run_insertion_test(
        black(4, 3, red_leaf(2, 7), None),
        3,
        5,
        black(3, 5, red_leaf(2, 7), red_leaf(4, 3)),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_left_side_recurse_red() {
    run_insertion_test(
        black(5, 7, black(3, 1, red_leaf(2, 6), red_leaf(4, 8)), black_leaf(9, 4)),
        1,
        5,
        black(
            5,
            7,
            red(3, 1, black(2, 6, red_leaf(1, 5), None), black_leaf(4, 8)),
            black_leaf(9, 4),
        ),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_left_side_recurse_black() {
    run_insertion_test(
        black(5, 7, black_leaf(3, 1), black_leaf(9, 4)),
        2,
        5,
        black(5, 7, black(3, 1, red_leaf(2, 5), None), black_leaf(9, 4)),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_left_side_equal() {
    run_insertion_test(
        black(5, 7, red_leaf(3, 1), red_leaf(9, 4)),
        3,
        2,
        black(5, 7, red_leaf(3, 2), red_leaf(9, 4)),
        Some(1),
    );
}

#[test]
fn test_red_black_tree_insert_root_right_side_case_1() {
    run_insertion_test(
        black(4, 3, red_leaf(1, 9), red_leaf(6, 7)),
        7,
        5,
        black(4, 3, black_leaf(1, 9), black(6, 7, None, red_leaf(7, 5))),
        None,
    );

    run_insertion_test(
        black(4, 3, red_leaf(1, 9), red_leaf(6, 7)),
        5,
        5,
        black(4, 3, black_leaf(1, 9), black(6, 7, red_leaf(5, 5), None)),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_right_side_case_2_and_3() {
    run_insertion_test(
        black(1, 3, None, red_leaf(3, 7)),
        4,
        5,
        black(3, 7, red_leaf(1, 3), red_leaf(4, 5)),
        None,
    );

    run_insertion_test(
        black(1, 3, None, red_leaf(3, 7)),
        2,
        5,
        black(2, 5, red_leaf(1, 3), red_leaf(3, 7)),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_right_side_recurse_red() {
    run_insertion_test(
        black(5, 7, black_leaf(1, 4), black(7, 1, red_leaf(6, 8), red_leaf(8, 6))),
        9,
        5,
        black(
            5,
            7,
            black_leaf(1, 4),
            red(7, 1, black_leaf(6, 8), black(8, 6, None, red_leaf(9, 5))),
        ),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_right_side_recurse_black() {
    run_insertion_test(
        black(6, 7, black_leaf(2, 4), black_leaf(8, 1)),
        9,
        5,
        black(6, 7, black_leaf(2, 4), black(8, 1, None, red_leaf(9, 5))),
        None,
    );
}

#[test]
fn test_red_black_tree_insert_root_right_side_equal() {
    run_insertion_test(
        black(7, 7, red_leaf(3, 4), red_leaf(9, 1)),
        9,
        2,
        black(7, 7, red_leaf(3, 4), red_leaf(9, 2)),
        Some(1),
    );
}

#[test]
fn red_black_tree_insert_full_left_side() {
    run_insertion_test(
        black(
            11,
            2,
            red(2, 3, black_leaf(1, 5), black(7, 7, red_leaf(5, 11), red_leaf(8, 13))),
            black(14, 17, None, red_leaf(15, 19)),
        ),
        4,
        23,
        black(
            7,
            7,
            red(2, 3, black_leaf(1, 5), black(5, 11, red_leaf(4, 23), None)),
            red(11, 2, black_leaf(8, 13), black(14, 17, None, red_leaf(15, 19))),
        ),
        None,
    )
}

#[test]
fn red_black_tree_insert_full_right_side() {
    run_insertion_test(
        black(
            5,
            2,
            black(2, 3, red_leaf(1, 5), None),
            red(
                14,
                7,
                black(9, 11, red_leaf(8, 13), red(11, 17, None, None)),
                black_leaf(15, 19),
            ),
        ),
        12,
        23,
        black(
            9,
            11,
            red(5, 2, black(2, 3, red_leaf(1, 5), None), black_leaf(8, 13)),
            red(14, 7, black(11, 17, None, red_leaf(12, 23)), black_leaf(15, 19)),
        ),
        None,
    )
}

// Deletion tests.

#[test]
fn test_adjust_on_left_child_case_1_minimal() {
    let mut tree = black(1, 2, None, red(3, 5, black_leaf(2, 3), black_leaf(4, 7)));

    assert!(!adjust_on_left_child(tree.as_mut().unwrap()));

    assert_eq!(tree, black(3, 5, black(1, 2, None, red_leaf(2, 3)), black_leaf(4, 7)));
}

#[test]
fn test_adjust_on_left_child_case_1_full() {
    let mut tree = black(
        2,
        3,
        black_leaf(1, 2),
        red(
            6,
            13,
            black(4, 7, black_leaf(3, 5), black_leaf(5, 11)),
            black(8, 19, black_leaf(7, 17), black_leaf(9, 23)),
        ),
    );

    assert!(!adjust_on_left_child(tree.as_mut().unwrap()));

    assert_eq!(
        tree,
        black(
            6,
            13,
            black(2, 3, black_leaf(1, 2), red(4, 7, black_leaf(3, 5), black_leaf(5, 11))),
            black(8, 19, black_leaf(7, 17), black_leaf(9, 23))
        )
    );
}

#[test]
fn test_adjust_on_left_child_case_2_red_root_minimal() {
    let red_tree = red(1, 2, None, black(3, 5, None, None));
    let expected_red_tree_result = black(1, 2, None, red(3, 5, None, None));

    for f in &[
        adjust_on_left_child_black_sibling_red_root,
        adjust_on_left_child_black_sibling,
        adjust_on_left_child,
    ] {
        let mut tree = red_tree.clone();

        assert!(!f(tree.as_mut().unwrap()));

        assert_eq!(&tree, &expected_red_tree_result);
    }
}

#[test]
fn test_red_black_tree_map() {
    let mut map = RedBlackTreeMap::new();

    assert_eq!(map.get(&4), None);
    assert_eq!(map.get(&5), None);

    assert_eq!(map.insert(4, 7), None);

    assert_eq!(map.get(&4), Some(&7));
    assert_eq!(map.get(&5), None);

    assert_eq!(map.insert(4, 8), Some(7));

    assert_eq!(map.get(&4), Some(&8));
    assert_eq!(map.get(&5), None);
}
