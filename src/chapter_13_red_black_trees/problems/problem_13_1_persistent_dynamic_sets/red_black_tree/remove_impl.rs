use super::{BlackNode, NodeContent, RedBlackTree, RedNode, RedOrBlackNode};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::rc::Rc;

fn rebalance_left_red<K, V>(
    node_content: NodeContent<K, V>,
    right: &BlackNode<K, V>,
    new_left: RedBlackTree<K, V>,
) -> RedOrBlackNode<K, V> {
    match &right.right {
        RedOrBlackNode::Red(right_right) => RedNode::new_rc(
            right.content.clone(),
            BlackNode::new_rc(node_content, new_left, right.left.clone()),
            right_right.with_black_color_rc(),
        )
        .into(),
        RedOrBlackNode::Black(right_right) => match &right.left {
            RedOrBlackNode::Red(right_left) => RedNode::new_rc(
                right_left.content.clone(),
                BlackNode::new_rc(node_content, new_left, right_left.left.clone()),
                right.with_left_rc(right_left.right.clone()),
            )
            .into(),
            RedOrBlackNode::Black(right_left) => BlackNode::new_rc(
                node_content,
                new_left,
                RedNode::new_rc(right.content.clone(), right_left.clone(), right_right.clone()),
            )
            .into(),
        },
    }
}

fn rebalance_left_black<K, V>(
    node_content: NodeContent<K, V>,
    right: &RedOrBlackNode<K, V>,
    new_left: RedBlackTree<K, V>,
) -> (Rc<BlackNode<K, V>>, bool) {
    match right {
        RedOrBlackNode::Red(right) => (
            BlackNode::new_rc(
                right.content.clone(),
                rebalance_left_red(node_content, right.left.as_ref().unwrap(), new_left),
                right.right.clone(),
            ),
            false,
        ),
        RedOrBlackNode::Black(right) => {
            let right = right.as_ref().unwrap();

            match &right.right {
                RedOrBlackNode::Red(right_right) => (
                    BlackNode::new_rc(
                        right.content.clone(),
                        BlackNode::new_rc(node_content, new_left, right.left.clone()),
                        right_right.with_black_color_rc(),
                    ),
                    false,
                ),
                RedOrBlackNode::Black(right_right) => match &right.left {
                    RedOrBlackNode::Red(right_left) => (
                        BlackNode::new_rc(
                            right_left.content.clone(),
                            BlackNode::new_rc(node_content, new_left, right_left.left.clone()),
                            right.with_left_rc(right_left.right.clone()),
                        ),
                        false,
                    ),
                    RedOrBlackNode::Black(right_left) => (
                        BlackNode::new_rc(
                            node_content,
                            new_left,
                            RedNode::new_rc(right.content.clone(), right_left.clone(), right_right.clone()),
                        ),
                        true,
                    ),
                },
            }
        }
    }
}

fn rebalance_right_red<K, V>(
    node_content: NodeContent<K, V>,
    left: &BlackNode<K, V>,
    new_right: RedBlackTree<K, V>,
) -> RedOrBlackNode<K, V> {
    match &left.left {
        RedOrBlackNode::Red(left_left) => RedNode::new_rc(
            left.content.clone(),
            left_left.with_black_color_rc(),
            BlackNode::new_rc(node_content, new_right, left.right.clone()),
        )
        .into(),
        RedOrBlackNode::Black(left_left) => match &left.right {
            RedOrBlackNode::Red(left_right) => RedNode::new_rc(
                left_right.content.clone(),
                left.with_right_rc(left_right.left.clone()),
                BlackNode::new_rc(node_content, new_right, left_right.right.clone()),
            )
            .into(),
            RedOrBlackNode::Black(left_right) => BlackNode::new_rc(
                node_content,
                RedNode::new_rc(left.content.clone(), left_right.clone(), left_left.clone()),
                new_right,
            )
            .into(),
        },
    }
}

fn rebalance_right_black<K, V>(
    node_content: NodeContent<K, V>,
    left: &RedOrBlackNode<K, V>,
    new_right: RedBlackTree<K, V>,
) -> (Rc<BlackNode<K, V>>, bool) {
    match left {
        RedOrBlackNode::Red(left) => (
            BlackNode::new_rc(
                left.content.clone(),
                left.left.clone(),
                rebalance_right_red(node_content, left.right.as_ref().unwrap(), new_right),
            ),
            false,
        ),
        RedOrBlackNode::Black(left) => {
            let left = left.as_ref().unwrap();

            match &left.left {
                RedOrBlackNode::Red(left_left) => (
                    BlackNode::new_rc(
                        left.content.clone(),
                        left_left.with_black_color_rc(),
                        BlackNode::new_rc(node_content, new_right, left.right.clone()),
                    ),
                    false,
                ),
                RedOrBlackNode::Black(left_left) => match &left.right {
                    RedOrBlackNode::Red(left_right) => (
                        BlackNode::new_rc(
                            left_right.content.clone(),
                            left.with_right_rc(left_right.left.clone()),
                            BlackNode::new_rc(node_content, left_right.right.clone(), new_right),
                        ),
                        false,
                    ),
                    RedOrBlackNode::Black(left_right) => (
                        BlackNode::new_rc(
                            node_content,
                            RedNode::new_rc(left.content.clone(), left_left.clone(), left_right.clone()),
                            new_right,
                        ),
                        true,
                    ),
                },
            }
        }
    }
}

fn remove_min_red<K, V>(node: &RedNode<K, V>) -> (RedOrBlackNode<K, V>, NodeContent<K, V>) {
    node.left.as_deref().map_or_else(
        || (None.into(), node.content.clone()),
        |left| match remove_min_black(left) {
            (new_left, min_node_content, false) => (node.with_left_rc(new_left).into(), min_node_content),
            (new_left, min_node_content, true) => (
                rebalance_left_red(node.content.clone(), node.right.as_ref().unwrap(), new_left),
                min_node_content,
            ),
        },
    )
}

fn remove_min_black<K, V>(node: &BlackNode<K, V>) -> (RedBlackTree<K, V>, NodeContent<K, V>, bool) {
    match &node.left {
        RedOrBlackNode::Red(left) => {
            let (new_left, min_node_content) = remove_min_red(left);

            (node.with_left_rc(new_left).into(), min_node_content, false)
        }
        RedOrBlackNode::Black(None) => match &node.right {
            RedOrBlackNode::Red(right) => (Some(right.with_black_color_rc()), node.content.clone(), false),
            RedOrBlackNode::Black(_) => (None, node.content.clone(), true),
        },
        RedOrBlackNode::Black(Some(left)) => match remove_min_black(left) {
            (new_left, min_node_content, false) => (Some(node.with_left_rc(new_left)), min_node_content, false),
            (new_left, min_node_content, true) => {
                let (new_node, black_height_reduced) =
                    rebalance_left_black(node.content.clone(), &node.right, new_left);

                (Some(new_node), min_node_content, black_height_reduced)
            }
        },
    }
}

enum RemoveBlackResult<K, V> {
    BlackHeightUnchanged(Option<(RedBlackTree<K, V>, Rc<V>)>),
    BlackHeightReduced(RedBlackTree<K, V>, Rc<V>),
}

fn remove_red<K: Borrow<Q>, V, Q: Ord + ?Sized>(
    node: &RedNode<K, V>,
    key: &Q,
) -> Option<(RedOrBlackNode<K, V>, Rc<V>)> {
    match key.cmp((*node.content.key).borrow()) {
        Ordering::Less => match remove_black(&node.left, key) {
            RemoveBlackResult::BlackHeightUnchanged(result) => {
                result.map(|(new_left, old_value)| (new_left.into(), old_value))
            }
            RemoveBlackResult::BlackHeightReduced(new_left, old_value) => Some((
                rebalance_left_red(node.content.clone(), node.right.as_ref().unwrap(), new_left),
                old_value,
            )),
        },
        Ordering::Equal => {
            if let (Some(left), Some(right)) = (&node.left, &node.right) {
                match remove_min_black(right) {
                    (new_right, min_content, false) => Some((
                        RedNode::new_rc(min_content, Rc::clone(left), new_right).into(),
                        Rc::clone(&node.content.value),
                    )),
                    (new_right, min_content, true) => Some((
                        rebalance_right_red(min_content, left, new_right),
                        Rc::clone(&node.content.value),
                    )),
                }
            } else {
                Some((None.into(), Rc::clone(&node.content.value)))
            }
        }
        Ordering::Greater => match remove_black(&node.right, key) {
            RemoveBlackResult::BlackHeightUnchanged(result) => {
                result.map(|(new_right, old_value)| (new_right.into(), old_value))
            }
            RemoveBlackResult::BlackHeightReduced(new_right, old_value) => Some((
                rebalance_right_red(node.content.clone(), node.left.as_ref().unwrap(), new_right),
                old_value,
            )),
        },
    }
}

fn remove_black<K: Borrow<Q>, V, Q: Ord + ?Sized>(tree: &RedBlackTree<K, V>, key: &Q) -> RemoveBlackResult<K, V> {
    if let Some(node) = tree {
        match key.cmp((*node.content.key).borrow()) {
            Ordering::Less => match &node.left {
                RedOrBlackNode::Red(left) => RemoveBlackResult::BlackHeightUnchanged(
                    remove_red(left, key).map(|(new_left, old_value)| (Some(node.with_left_rc(new_left)), old_value)),
                ),
                RedOrBlackNode::Black(left) => match remove_black(left, key) {
                    RemoveBlackResult::BlackHeightUnchanged(result) => RemoveBlackResult::BlackHeightUnchanged(
                        result.map(|(new_left, old_value)| (Some(node.with_left_rc(new_left)), old_value)),
                    ),
                    RemoveBlackResult::BlackHeightReduced(new_left, old_value) => {
                        match rebalance_left_black(node.content.clone(), &node.right, new_left) {
                            (new_node, false) => {
                                RemoveBlackResult::BlackHeightUnchanged(Some((Some(new_node), old_value)))
                            }
                            (new_node, true) => RemoveBlackResult::BlackHeightReduced(Some(new_node), old_value),
                        }
                    }
                },
            },
            Ordering::Equal => match (&node.left, &node.right) {
                (RedOrBlackNode::Black(None), RedOrBlackNode::Black(None)) => {
                    RemoveBlackResult::BlackHeightReduced(None, Rc::clone(&node.content.value))
                }
                (RedOrBlackNode::Black(None), RedOrBlackNode::Red(right)) => RemoveBlackResult::BlackHeightUnchanged(
                    Some((Some(right.with_black_color_rc()), Rc::clone(&node.content.value))),
                ),
                (RedOrBlackNode::Red(left), RedOrBlackNode::Black(None)) => RemoveBlackResult::BlackHeightUnchanged(
                    Some((Some(left.with_black_color_rc()), Rc::clone(&node.content.value))),
                ),
                (left, RedOrBlackNode::Red(right)) => {
                    let (new_right, min_node_content) = remove_min_red(right);

                    RemoveBlackResult::BlackHeightUnchanged(Some((
                        Some(BlackNode::new_rc(min_node_content, left.clone(), new_right)),
                        Rc::clone(&node.content.value),
                    )))
                }
                (left, RedOrBlackNode::Black(Some(right))) => match remove_min_black(right) {
                    (new_right, min_content, false) => RemoveBlackResult::BlackHeightUnchanged(Some((
                        BlackNode::new_rc(min_content, left.clone(), new_right).into(),
                        Rc::clone(&node.content.value),
                    ))),
                    (new_right, min_content, true) => match rebalance_right_black(min_content, left, new_right) {
                        (new_node, false) => RemoveBlackResult::BlackHeightUnchanged(Some((
                            Some(new_node),
                            Rc::clone(&node.content.value),
                        ))),
                        (new_node, true) => {
                            RemoveBlackResult::BlackHeightReduced(Some(new_node), Rc::clone(&node.content.value))
                        }
                    },
                },
                _ => unreachable!(),
            },
            Ordering::Greater => match &node.right {
                RedOrBlackNode::Red(right) => RemoveBlackResult::BlackHeightUnchanged(
                    remove_red(right, key)
                        .map(|(new_right, old_value)| (Some(node.with_right_rc(new_right)), old_value)),
                ),
                RedOrBlackNode::Black(right) => match remove_black(right, key) {
                    RemoveBlackResult::BlackHeightUnchanged(result) => RemoveBlackResult::BlackHeightUnchanged(
                        result.map(|(new_right, old_value)| (Some(node.with_right_rc(new_right)), old_value)),
                    ),
                    RemoveBlackResult::BlackHeightReduced(new_right, old_value) => {
                        match rebalance_right_black(node.content.clone(), &node.left, new_right) {
                            (new_node, false) => {
                                RemoveBlackResult::BlackHeightUnchanged(Some((Some(new_node), old_value)))
                            }
                            (new_node, true) => RemoveBlackResult::BlackHeightReduced(Some(new_node), old_value),
                        }
                    }
                },
            },
        }
    } else {
        RemoveBlackResult::BlackHeightUnchanged(None)
    }
}

pub fn persistent_red_black_tree_remove<K: Borrow<Q>, V, Q: Ord + ?Sized>(
    tree: &RedBlackTree<K, V>,
    key: &Q,
) -> Option<(RedBlackTree<K, V>, Rc<V>)> {
    match remove_black(tree, key) {
        RemoveBlackResult::BlackHeightUnchanged(result) => result,
        RemoveBlackResult::BlackHeightReduced(new_node, old_value) => Some((new_node, old_value)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{black, black_leaf, red, red_leaf};
    use super::super::{BlackNode, RedBlackTree};
    use std::borrow::Borrow;
    use std::rc::Rc;

    fn remove<K: Borrow<Q>, V, T: Into<RedBlackTree<K, V>>, Q: Ord>(
        tree: T,
        key: &Q,
    ) -> Option<(RedBlackTree<K, V>, Rc<V>)> {
        super::persistent_red_black_tree_remove(&tree.into(), key)
    }

    #[test]
    fn test_remove_not_exist() {
        assert_eq!(remove(None::<Rc<BlackNode<i32, i32>>>, &4), None);
        assert_eq!(remove(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), &1), None);
        assert_eq!(remove(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), &3), None);
        assert_eq!(remove(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), &5), None);
        assert_eq!(remove(black(4, 3, red_leaf(2, 2), red_leaf(6, 5)), &7), None);
    }

    #[test]
    fn test_remove_case_1() {
        assert_eq!(
            remove(
                black(2, 3, black_leaf(1, 2), red(4, 7, black_leaf(3, 5), black_leaf(5, 11))),
                &1
            ),
            Some((
                Some(black(4, 7, black(2, 3, None, red_leaf(3, 5)), black_leaf(5, 11))),
                2.into()
            ))
        );

        assert_eq!(
            remove(
                black(4, 3, red(2, 7, black_leaf(1, 11), black_leaf(3, 5)), black_leaf(5, 2)),
                &5
            ),
            Some((
                Some(black(2, 7, black_leaf(1, 11), black(4, 3, red_leaf(3, 5), None))),
                2.into()
            ))
        );
    }

    #[test]
    fn test_remove_case_2() {
        assert_eq!(
            remove(black(2, 3, black_leaf(1, 2), black_leaf(3, 5)), &1),
            Some((Some(black(2, 3, None, red_leaf(3, 5))), 2.into()))
        );

        assert_eq!(
            remove(
                black(2, 3, black_leaf(1, 2), red(4, 7, black_leaf(3, 5), black_leaf(5, 11))),
                &3
            ),
            Some((
                Some(black(2, 3, black_leaf(1, 2), black(4, 7, None, red_leaf(5, 11)))),
                5.into()
            ))
        );

        assert_eq!(
            remove(black(2, 3, black_leaf(1, 5), black_leaf(3, 2)), &3),
            Some((Some(black(2, 3, red_leaf(1, 5), None)), 2.into()))
        );

        assert_eq!(
            remove(
                black(4, 3, red(2, 7, black_leaf(1, 11), black_leaf(3, 5)), black_leaf(5, 2)),
                &3
            ),
            Some((
                Some(black(4, 3, black(2, 7, red_leaf(1, 11), None), black_leaf(5, 2))),
                5.into()
            ))
        );
    }

    #[test]
    fn test_remove_case_3() {
        assert_eq!(
            remove(black(2, 3, black_leaf(1, 2), black(4, 7, red_leaf(3, 5), None)), &1),
            Some((Some(black(3, 5, black_leaf(2, 3), black_leaf(4, 7))), 2.into()))
        );

        assert_eq!(
            remove(
                black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(4, 7, black_leaf(3, 5), black(6, 13, red_leaf(5, 11), None))
                ),
                &3
            ),
            Some((
                Some(black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(5, 11, black_leaf(4, 7), black_leaf(6, 13))
                )),
                5.into()
            ))
        );

        assert_eq!(
            remove(black(3, 3, black(1, 7, None, red_leaf(2, 5)), black_leaf(4, 2)), &4),
            Some((Some(black(2, 5, black_leaf(1, 7), black_leaf(3, 3))), 2.into()))
        );

        assert_eq!(
            remove(
                black(
                    5,
                    3,
                    red(3, 7, black(1, 13, None, red_leaf(2, 11)), black_leaf(4, 5)),
                    black_leaf(6, 2),
                ),
                &4
            ),
            Some((
                Some(black(
                    5,
                    3,
                    red(2, 11, black_leaf(1, 13), black_leaf(3, 7)),
                    black_leaf(6, 2),
                )),
                5.into()
            ))
        );
    }

    #[test]
    fn test_remove_case_4() {
        assert_eq!(
            remove(black(2, 3, black_leaf(1, 2), black(3, 5, None, red_leaf(4, 7))), &1),
            Some((Some(black(3, 5, black_leaf(2, 3), black_leaf(4, 7))), 2.into()))
        );

        assert_eq!(
            remove(
                black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(4, 7, black_leaf(3, 5), black(5, 11, None, red_leaf(6, 13)))
                ),
                &3
            ),
            Some((
                Some(black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(5, 11, black_leaf(4, 7), black_leaf(6, 13))
                )),
                5.into()
            ))
        );

        assert_eq!(
            remove(black(3, 3, black(2, 5, red_leaf(1, 7), None), black_leaf(4, 2)), &4),
            Some((Some(black(2, 5, black_leaf(1, 7), black_leaf(3, 3))), 2.into()))
        );

        assert_eq!(
            remove(
                black(
                    5,
                    3,
                    red(3, 7, black(2, 11, red_leaf(1, 13), None), black_leaf(4, 5)),
                    black_leaf(6, 2),
                ),
                &4
            ),
            Some((
                Some(black(
                    5,
                    3,
                    red(2, 11, black_leaf(1, 13), black_leaf(3, 7)),
                    black_leaf(6, 2),
                )),
                5.into()
            ))
        );
    }

    #[test]
    fn test_remove_right_red() {
        assert_eq!(
            remove(
                black(2, 3, black_leaf(1, 2), red(4, 7, black_leaf(3, 5), black_leaf(5, 11))),
                &2
            ),
            Some((
                Some(black(3, 5, black_leaf(1, 2), black(4, 7, None, red_leaf(5, 11)))),
                3.into()
            ))
        );
    }

    #[test]
    fn test_remove_right_black() {
        assert_eq!(
            remove(
                black(
                    4,
                    7,
                    black(2, 3, red_leaf(1, 2), red_leaf(3, 5)),
                    black(6, 13, red_leaf(5, 11), red_leaf(7, 17))
                ),
                &4
            ),
            Some((
                Some(black(
                    5,
                    11,
                    black(2, 3, red_leaf(1, 2), red_leaf(3, 5)),
                    black(6, 13, None, red_leaf(7, 17))
                )),
                7.into()
            ))
        );

        assert_eq!(
            remove(
                black(
                    4,
                    7,
                    black(2, 3, black_leaf(1, 2), black_leaf(3, 5)),
                    black(6, 13, black_leaf(5, 11), black_leaf(7, 17))
                ),
                &4
            ),
            Some((
                Some(black(
                    5,
                    11,
                    red(2, 3, black_leaf(1, 2), black_leaf(3, 5)),
                    black(6, 13, None, red_leaf(7, 17))
                )),
                7.into()
            ))
        );
    }

    #[test]
    fn test_remove_red_node() {
        assert_eq!(
            remove(black(2, 3, red_leaf(1, 2), red_leaf(3, 5)), &1),
            Some((Some(black(2, 3, None, red_leaf(3, 5))), 2.into()))
        );

        assert_eq!(
            remove(
                black(2, 3, black_leaf(1, 2), red(4, 7, black_leaf(3, 5), black_leaf(5, 11))),
                &4
            ),
            Some((
                Some(black(2, 3, black_leaf(1, 2), black(5, 11, red_leaf(3, 5), None))),
                7.into()
            ))
        );

        assert_eq!(
            remove(
                black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(4, 7, black_leaf(3, 5), black(6, 13, red_leaf(5, 11), None))
                ),
                &4
            ),
            Some((
                Some(black(
                    2,
                    3,
                    black_leaf(1, 2),
                    red(5, 11, black_leaf(3, 5), black_leaf(6, 13))
                )),
                7.into()
            ))
        );

        assert_eq!(
            remove(black(3, 5, black(2, 3, red_leaf(1, 2), None), black_leaf(4, 7)), &1),
            Some((Some(black(3, 5, black_leaf(2, 3), black_leaf(4, 7))), 2.into()))
        );

        assert_eq!(
            remove(black(2, 5, black_leaf(1, 7), black(3, 3, None, red_leaf(4, 2))), &4),
            Some((Some(black(2, 5, black_leaf(1, 7), black_leaf(3, 3))), 2.into()))
        );
    }

    #[test]
    fn test_remove_black_node() {
        assert_eq!(
            remove(black(2, 3, None, red_leaf(3, 5)), &2),
            Some((Some(black_leaf(3, 5)), 3.into()))
        );

        assert_eq!(
            remove(black(2, 3, red_leaf(1, 2), None), &2),
            Some((Some(black_leaf(1, 2)), 3.into()))
        );

        assert_eq!(
            remove(black(2, 3, black_leaf(1, 2), black_leaf(3, 5)), &2),
            Some((Some(black(3, 5, red_leaf(1, 2), None)), 3.into()))
        );

        assert_eq!(
            remove(
                black(4, 7, red(2, 3, black_leaf(1, 2), black_leaf(3, 5)), black_leaf(5, 11)),
                &4
            ),
            Some((
                Some(black(2, 3, black_leaf(1, 2), black(5, 11, red_leaf(3, 5), None))),
                7.into()
            ))
        );
    }
}
