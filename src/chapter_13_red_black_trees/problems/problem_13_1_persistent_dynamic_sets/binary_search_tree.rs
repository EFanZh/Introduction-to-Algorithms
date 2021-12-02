use std::borrow::Borrow;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct Node<K, V> {
    key: Rc<K>,
    value: Rc<V>,
    left: Option<Rc<Self>>,
    right: Option<Rc<Self>>,
}

type Tree<K, V> = Option<Rc<Node<K, V>>>;

impl<K, V> Node<K, V> {
    fn new(key: Rc<K>, value: Rc<V>, left: Tree<K, V>, right: Tree<K, V>) -> Self {
        Self {
            key,
            value,
            left,
            right,
        }
    }

    fn new_leaf(key: Rc<K>, value: Rc<V>) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    fn with_value(&self, value: Rc<V>) -> Self {
        Self {
            key: Rc::clone(&self.key),
            value,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }

    fn with_left(&self, left: Option<Rc<Self>>) -> Self {
        Self {
            key: Rc::clone(&self.key),
            value: Rc::clone(&self.value),
            left,
            right: self.right.clone(),
        }
    }

    fn with_right(&self, right: Option<Rc<Self>>) -> Self {
        Self {
            key: Rc::clone(&self.key),
            value: Rc::clone(&self.value),
            left: self.left.clone(),
            right,
        }
    }
}

pub fn persistent_tree_insert<K: Ord, V>(
    tree: &Tree<K, V>,
    key: Rc<K>,
    value: Rc<V>,
) -> (Rc<Node<K, V>>, Option<Rc<V>>) {
    if let Some(node) = tree {
        match key.cmp(&node.key) {
            Ordering::Less => {
                let (new_left, old_value) = persistent_tree_insert(&node.left, key, value);

                (Rc::new(node.with_left(Some(new_left))), old_value)
            }
            Ordering::Equal => (Rc::new(node.with_value(value)), Some(Rc::clone(&node.value))),
            Ordering::Greater => {
                let (new_right, old_value) = persistent_tree_insert(&node.right, key, value);

                (Rc::new(node.with_right(Some(new_right))), old_value)
            }
        }
    } else {
        (Rc::new(Node::new_leaf(key, value)), None)
    }
}

pub fn persistent_tree_search<'a, K: Ord + Borrow<Q>, V, Q: Ord + ?Sized>(
    mut tree: &'a Tree<K, V>,
    key: &Q,
) -> Option<&'a Rc<V>> {
    while let Some(node) = tree {
        match key.cmp((*node.key).borrow()) {
            Ordering::Less => tree = &node.left,
            Ordering::Equal => return Some(&node.value),
            Ordering::Greater => tree = &node.right,
        }
    }

    None
}

fn persistent_tree_remove_min<K, V>(tree: &Node<K, V>) -> (Tree<K, V>, (Rc<K>, Rc<V>)) {
    tree.left.as_deref().map_or_else(
        || (tree.right.clone(), (Rc::clone(&tree.key), Rc::clone(&tree.value))),
        |left| {
            let (new_left, (min_key, min_value)) = persistent_tree_remove_min(left);

            (Some(Rc::new(tree.with_left(new_left))), (min_key, min_value))
        },
    )
}

pub fn persistent_tree_remove<K: Ord + Borrow<Q>, V, Q: Ord + ?Sized>(
    tree: &Tree<K, V>,
    key: &Q,
) -> Option<(Tree<K, V>, Rc<V>)> {
    tree.as_ref().and_then(|node| match key.cmp((*node.key).borrow()) {
        Ordering::Less => persistent_tree_remove(&node.left, key)
            .map(|(new_left, old_value)| (Some(Rc::new(node.with_left(new_left))), old_value)),
        Ordering::Equal => {
            let new_tree = node.left.as_ref().map_or_else(
                || node.right.clone(),
                |left| {
                    Some(node.right.as_deref().map_or_else(
                        || Rc::clone(left),
                        |right| {
                            let (new_right, (min_key, min_value)) = persistent_tree_remove_min(right);

                            Rc::new(Node::new(min_key, min_value, Some(Rc::clone(left)), new_right))
                        },
                    ))
                },
            );

            Some((new_tree, Rc::clone(&node.value)))
        }
        Ordering::Greater => persistent_tree_remove(&node.right, key)
            .map(|(new_right, old_value)| (Some(Rc::new(node.with_right(new_right))), old_value)),
    })
}

#[cfg(test)]
mod tests {
    use super::{persistent_tree_insert, persistent_tree_remove, persistent_tree_search, Node};
    use std::rc::Rc;

    #[test]
    fn test_persistent_tree_insert_empty() {
        assert_eq!(
            persistent_tree_insert(&None, 1.into(), 2.into()),
            (Node::new_leaf(1.into(), 2.into()).into(), None)
        );
    }

    #[test]
    fn test_persistent_tree_insert_less() {
        assert_eq!(
            persistent_tree_insert(&Some(Node::new_leaf(1.into(), 2.into()).into()), 0.into(), 3.into()),
            (
                Node::new(
                    1.into(),
                    2.into(),
                    Some(Node::new_leaf(0.into(), 3.into()).into()),
                    None
                )
                .into(),
                None
            )
        );
    }

    #[test]
    fn test_persistent_tree_insert_equal() {
        assert_eq!(
            persistent_tree_insert(&Some(Node::new_leaf(1.into(), 2.into()).into()), 1.into(), 3.into()),
            (Node::new_leaf(1.into(), 3.into()).into(), Some(2.into()))
        );
    }

    #[test]
    fn test_persistent_tree_insert_greater() {
        assert_eq!(
            persistent_tree_insert(&Some(Node::new_leaf(1.into(), 2.into()).into()), 2.into(), 3.into()),
            (
                Node::new(
                    1.into(),
                    2.into(),
                    None,
                    Some(Node::new_leaf(2.into(), 3.into()).into())
                )
                .into(),
                None
            )
        );
    }

    #[test]
    fn test_persistent_tree_search_empty() {
        let tree: Option<Rc<Node<i32, i32>>> = None;

        assert_eq!(persistent_tree_search(&tree, &1), None);
    }

    #[test]
    fn test_persistent_tree_search_less() {
        assert_eq!(
            persistent_tree_search(&Some(Node::new_leaf(1.into(), 2.into()).into()), &0),
            None
        );
    }

    #[test]
    fn test_persistent_tree_search_equal() {
        assert_eq!(
            persistent_tree_search(&Some(Node::new_leaf(1.into(), 2.into()).into()), &1),
            Some(&2.into())
        );
    }

    #[test]
    fn test_persistent_tree_search_more() {
        assert_eq!(
            persistent_tree_search(&Some(Node::new_leaf(1.into(), 2.into()).into()), &3),
            None
        );
    }

    #[test]
    fn test_persistent_tree_search_left() {
        assert_eq!(
            persistent_tree_search(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        Some(Node::new_leaf(0.into(), 3.into()).into()),
                        Some(Node::new_leaf(2.into(), 5.into()).into())
                    )
                    .into()
                ),
                &0
            ),
            Some(&3.into())
        );
    }

    #[test]
    fn test_persistent_tree_search_right() {
        assert_eq!(
            persistent_tree_search(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        Some(Node::new_leaf(0.into(), 3.into()).into()),
                        Some(Node::new_leaf(2.into(), 5.into()).into())
                    )
                    .into()
                ),
                &2
            ),
            Some(&5.into())
        );
    }

    #[test]
    fn test_persistent_tree_remove_empty() {
        let tree: Option<Rc<Node<i32, i32>>> = None;

        assert_eq!(persistent_tree_remove(&tree, &2), None);
    }

    #[test]
    fn test_persistent_tree_remove_left() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        3.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        Some(Node::new_leaf(2.into(), 5.into()).into())
                    )
                    .into()
                ),
                &0
            ),
            Some((
                Some(
                    Node::new(
                        1.into(),
                        3.into(),
                        None,
                        Some(Node::new_leaf(2.into(), 5.into()).into())
                    )
                    .into()
                ),
                4.into()
            ))
        );
    }

    #[test]
    fn test_persistent_tree_remove_right() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        3.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        Some(Node::new_leaf(2.into(), 5.into()).into())
                    )
                    .into()
                ),
                &2
            ),
            Some((
                Some(
                    Node::new(
                        1.into(),
                        3.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        None
                    )
                    .into()
                ),
                5.into()
            ))
        );
    }

    #[test]
    fn test_persistent_tree_remove_left_empty() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        None,
                        Some(Node::new_leaf(3.into(), 4.into()).into())
                    )
                    .into()
                ),
                &1
            ),
            Some((Some(Node::new_leaf(3.into(), 4.into()).into()), 2.into()))
        );
    }

    #[test]
    fn test_persistent_tree_remove_right_empty() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        None,
                    )
                    .into()
                ),
                &1
            ),
            Some((Some(Node::new_leaf(0.into(), 4.into()).into()), 2.into()))
        );
    }

    #[test]
    fn test_persistent_tree_remove_both_non_empty_1() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        Some(Node::new_leaf(2.into(), 5.into()).into()),
                    )
                    .into()
                ),
                &1
            ),
            Some((
                Some(
                    Node::new(
                        2.into(),
                        5.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        None,
                    )
                    .into()
                ),
                2.into()
            ))
        );
    }

    #[test]
    fn test_persistent_tree_remove_both_non_empty_2() {
        assert_eq!(
            persistent_tree_remove(
                &Some(
                    Node::new(
                        1.into(),
                        2.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        Some(
                            Node::new(
                                3.into(),
                                5.into(),
                                Some(Node::new_leaf(2.into(), 6.into()).into()),
                                Some(Node::new_leaf(4.into(), 7.into()).into()),
                            )
                            .into()
                        ),
                    )
                    .into()
                ),
                &1
            ),
            Some((
                Some(
                    Node::new(
                        2.into(),
                        6.into(),
                        Some(Node::new_leaf(0.into(), 4.into()).into()),
                        Some(
                            Node::new(
                                3.into(),
                                5.into(),
                                None,
                                Some(Node::new_leaf(4.into(), 7.into()).into()),
                            )
                            .into()
                        ),
                    )
                    .into()
                ),
                2.into()
            ))
        );
    }
}
