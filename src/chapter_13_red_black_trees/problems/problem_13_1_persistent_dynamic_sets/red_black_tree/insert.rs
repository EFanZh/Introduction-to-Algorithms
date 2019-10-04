use super::{BlackNode, NodeContent, RedBlackTree, RedNode, RedOrBlackNode};
use std::cmp::Ordering;
use std::rc::Rc;

enum RelaxedInsertResult<K, V> {
    Red(RedNode<K, V>),
    Black(BlackNode<K, V>, Option<Rc<V>>),
}

fn relaxed_insert<K: Ord, V>(node: &RedBlackTree<K, V>, key: Rc<K>, value: Rc<V>) -> RelaxedInsertResult<K, V> {
    if let Some(node) = node {
        match key.cmp(&node.content.key) {
            Ordering::Less => match &node.left {
                RedOrBlackNode::Red(left) => match key.cmp(&left.content.key) {
                    Ordering::Less => match relaxed_insert(&left.left, key, value) {
                        RelaxedInsertResult::Red(new_left_left) => match &node.right {
                            RedOrBlackNode::Red(right) => RelaxedInsertResult::Red(RedNode::new(
                                node.content.clone(),
                                BlackNode::new_rc(left.content.clone(), Rc::new(new_left_left), left.right.clone()),
                                right.with_black_color_rc(),
                            )),
                            RedOrBlackNode::Black(right) => RelaxedInsertResult::Black(
                                BlackNode::new(
                                    left.content.clone(),
                                    Rc::new(new_left_left),
                                    RedNode::new_rc(node.content.clone(), left.right.clone(), right.clone()),
                                ),
                                None,
                            ),
                        },
                        RelaxedInsertResult::Black(new_left_left, old_value) => RelaxedInsertResult::Black(
                            node.with_left(left.with_left_rc(Rc::new(new_left_left))),
                            old_value,
                        ),
                    },
                    Ordering::Equal => RelaxedInsertResult::Black(
                        node.with_left(left.with_value_rc(value)),
                        Some(left.content.value.clone()),
                    ),
                    Ordering::Greater => match relaxed_insert(&left.right, key, value) {
                        RelaxedInsertResult::Red(new_left_right) => match &node.right {
                            RedOrBlackNode::Red(right) => RelaxedInsertResult::Red(RedNode::new(
                                node.content.clone(),
                                BlackNode::new_rc(left.content.clone(), left.left.clone(), Rc::new(new_left_right)),
                                right.with_black_color_rc(),
                            )),
                            RedOrBlackNode::Black(right) => RelaxedInsertResult::Black(
                                BlackNode::new(
                                    new_left_right.content,
                                    left.with_right_rc(new_left_right.left),
                                    RedNode::new_rc(node.content.clone(), new_left_right.right, right.clone()),
                                ),
                                None,
                            ),
                        },
                        RelaxedInsertResult::Black(new_left_right, old_value) => RelaxedInsertResult::Black(
                            node.with_left(left.with_right_rc(Rc::new(new_left_right))),
                            old_value,
                        ),
                    },
                },
                RedOrBlackNode::Black(left) => match relaxed_insert(left, key, value) {
                    RelaxedInsertResult::Red(new_left) => {
                        RelaxedInsertResult::Black(node.with_left(Rc::new(new_left)), None)
                    }
                    RelaxedInsertResult::Black(new_left, old_value) => {
                        RelaxedInsertResult::Black(node.with_left(Rc::new(new_left)), old_value)
                    }
                },
            },
            Ordering::Equal => RelaxedInsertResult::Black(node.with_value(value), Some(node.content.value.clone())),
            Ordering::Greater => match &node.right {
                RedOrBlackNode::Red(right) => match key.cmp(&right.content.key) {
                    Ordering::Less => match relaxed_insert(&right.left, key, value) {
                        RelaxedInsertResult::Red(new_right_left) => match &node.left {
                            RedOrBlackNode::Red(left) => RelaxedInsertResult::Red(RedNode::new(
                                node.content.clone(),
                                left.with_black_color_rc(),
                                BlackNode::new_rc(right.content.clone(), Rc::new(new_right_left), right.right.clone()),
                            )),
                            RedOrBlackNode::Black(left) => RelaxedInsertResult::Black(
                                BlackNode::new(
                                    new_right_left.content,
                                    RedNode::new_rc(node.content.clone(), left.clone(), new_right_left.left),
                                    right.with_left_rc(new_right_left.right),
                                ),
                                None,
                            ),
                        },
                        RelaxedInsertResult::Black(new_right_left, old_value) => RelaxedInsertResult::Black(
                            node.with_right(right.with_left_rc(Rc::new(new_right_left))),
                            old_value,
                        ),
                    },
                    Ordering::Equal => RelaxedInsertResult::Black(
                        node.with_right(right.with_value_rc(value)),
                        Some(right.content.value.clone()),
                    ),
                    Ordering::Greater => match relaxed_insert(&right.right, key, value) {
                        RelaxedInsertResult::Red(new_right_right) => match &node.left {
                            RedOrBlackNode::Red(left) => RelaxedInsertResult::Red(RedNode::new(
                                node.content.clone(),
                                left.with_black_color_rc(),
                                BlackNode::new_rc(right.content.clone(), right.left.clone(), Rc::new(new_right_right)),
                            )),
                            RedOrBlackNode::Black(left) => RelaxedInsertResult::Black(
                                BlackNode::new(
                                    right.content.clone(),
                                    RedNode::new_rc(node.content.clone(), left.clone(), right.left.clone()),
                                    Rc::new(new_right_right),
                                ),
                                None,
                            ),
                        },
                        RelaxedInsertResult::Black(new_right_right, old_value) => RelaxedInsertResult::Black(
                            node.with_right(right.with_right_rc(Rc::new(new_right_right))),
                            old_value,
                        ),
                    },
                },
                RedOrBlackNode::Black(right) => match relaxed_insert(right, key, value) {
                    RelaxedInsertResult::Red(new_right) => {
                        RelaxedInsertResult::Black(node.with_right(Rc::new(new_right)), None)
                    }
                    RelaxedInsertResult::Black(new_right, old_value) => {
                        RelaxedInsertResult::Black(node.with_right(Rc::new(new_right)), old_value)
                    }
                },
            },
        }
    } else {
        RelaxedInsertResult::Red(RedNode::new_leaf(NodeContent { key, value }))
    }
}

pub fn persistent_red_black_tree_insert<K: Ord, V>(
    tree: &RedBlackTree<K, V>,
    key: Rc<K>,
    value: Rc<V>,
) -> (Rc<BlackNode<K, V>>, Option<Rc<V>>) {
    match relaxed_insert(tree, key, value) {
        RelaxedInsertResult::Red(new_node) => (new_node.with_black_color_rc(), None),
        RelaxedInsertResult::Black(new_node, old_value) => (Rc::new(new_node), old_value),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{black, black_leaf, red, red_leaf};
    use super::super::{BlackNode, RedBlackTree};
    use super::persistent_red_black_tree_insert;
    use std::rc::Rc;

    fn insert<K: Ord, V, T: Into<RedBlackTree<K, V>>>(
        tree: T,
        key: K,
        value: V,
    ) -> (Rc<BlackNode<K, V>>, Option<Rc<V>>) {
        persistent_red_black_tree_insert(&tree.into(), key.into(), value.into())
    }

    #[test]
    fn test_persistent_red_black_tree_insert_empty() {
        assert_eq!(insert(None, 1, 2), (black_leaf(1, 2), None));
    }

    #[test]
    fn test_persistent_red_black_tree_insert_into_black_node() {
        assert_eq!(insert(black_leaf(1, 2), 1, 3), (black_leaf(1, 3), Some(2.into())));
    }

    #[test]
    fn test_persistent_red_black_tree_insert_into_red() {
        assert_eq!(
            insert(black(2, 3, red_leaf(1, 2), red_leaf(3, 5)), 1, 7),
            (black(2, 3, red_leaf(1, 7), red_leaf(3, 5)), Some(2.into()))
        );

        assert_eq!(
            insert(black(2, 3, red_leaf(1, 2), red_leaf(3, 5)), 3, 7),
            (black(2, 3, red_leaf(1, 2), red_leaf(3, 7)), Some(5.into()))
        );
    }

    #[test]
    fn test_persistent_red_black_tree_insert_case_1() {
        assert_eq!(
            insert(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), 1, 7),
            (black(4, 3, black(2, 2, red_leaf(1, 7), None), black_leaf(6, 5)), None)
        );

        assert_eq!(
            insert(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), 3, 7),
            (black(4, 3, black(2, 2, None, red_leaf(3, 7)), black_leaf(6, 5)), None)
        );

        assert_eq!(
            insert(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), 5, 7),
            (black(4, 3, black_leaf(2, 2), black(6, 5, red_leaf(5, 7), None)), None)
        );

        assert_eq!(
            insert(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), 7, 7),
            (black(4, 3, black_leaf(2, 2), black(6, 5, None, red_leaf(7, 7))), None)
        );
    }

    #[test]
    fn test_persistent_red_black_tree_insert_case_2() {
        assert_eq!(
            insert(black(3, 5, red_leaf(1, 2), None), 2, 7),
            (black(2, 7, red_leaf(1, 2), red_leaf(3, 5)), None)
        );

        assert_eq!(
            insert(black(1, 2, None, red_leaf(3, 5)), 2, 7),
            (black(2, 7, red_leaf(1, 2), red_leaf(3, 5)), None)
        );
    }

    #[test]
    fn test_persistent_red_black_tree_insert_case_3() {
        assert_eq!(
            insert(black(3, 5, red_leaf(2, 7), None), 1, 2),
            (black(2, 7, red_leaf(1, 2), red_leaf(3, 5)), None)
        );

        assert_eq!(
            insert(black(1, 2, None, red_leaf(2, 7)), 3, 5),
            (black(2, 7, red_leaf(1, 2), red_leaf(3, 5)), None)
        );
    }

    #[test]
    fn test_persistent_red_black_tree_insert_no_adjust_1() {
        assert_eq!(
            insert(black_leaf(2, 3), 1, 4),
            (black(2, 3, red_leaf(1, 4), None), None)
        );

        assert_eq!(
            insert(black_leaf(2, 3), 3, 4),
            (black(2, 3, None, red_leaf(3, 4)), None)
        );

        assert_eq!(
            insert(black(2, 3, black_leaf(1, 4), black_leaf(3, 9)), 0, 7),
            (black(2, 3, black(1, 4, red_leaf(0, 7), None), black_leaf(3, 9)), None)
        );

        assert_eq!(
            insert(black(2, 3, black_leaf(1, 4), black_leaf(3, 9)), 4, 7),
            (black(2, 3, black_leaf(1, 4), black(3, 9, None, red_leaf(4, 7))), None)
        );
    }

    #[test]
    fn test_persistent_red_black_tree_insert_no_adjust_2() {
        let tree = black(
            8,
            88,
            red(4, 44, black_leaf(2, 22), black_leaf(6, 66)),
            red(12, 1212, black_leaf(10, 1010), black_leaf(14, 1414)),
        );

        assert_eq!(
            insert(tree.clone(), 1, 11),
            (
                black(
                    8,
                    88,
                    red(4, 44, black(2, 22, red_leaf(1, 11), None), black_leaf(6, 66)),
                    red(12, 1212, black_leaf(10, 1010), black_leaf(14, 1414)),
                ),
                None
            )
        );

        assert_eq!(
            insert(tree.clone(), 5, 55),
            (
                black(
                    8,
                    88,
                    red(4, 44, black_leaf(2, 22), black(6, 66, red_leaf(5, 55), None)),
                    red(12, 1212, black_leaf(10, 1010), black_leaf(14, 1414)),
                ),
                None
            )
        );

        assert_eq!(
            insert(tree.clone(), 9, 99),
            (
                black(
                    8,
                    88,
                    red(4, 44, black_leaf(2, 22), black_leaf(6, 66)),
                    red(12, 1212, black(10, 1010, red_leaf(9, 99), None), black_leaf(14, 1414)),
                ),
                None
            )
        );

        assert_eq!(
            insert(tree.clone(), 13, 1313),
            (
                black(
                    8,
                    88,
                    red(4, 44, black_leaf(2, 22), black_leaf(6, 66)),
                    red(
                        12,
                        1212,
                        black_leaf(10, 1010),
                        black(14, 1414, red_leaf(13, 1313), None)
                    ),
                ),
                None
            )
        );
    }
}
