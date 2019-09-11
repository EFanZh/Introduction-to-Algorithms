use std::borrow::Borrow;
use std::cmp::Ordering;
use std::mem;

// Basic definitions.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Black,
}

type Tree<K, V> = Option<Box<Node<K, V>>>;

#[derive(PartialEq, Eq, Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Tree<K, V>,
    right: Tree<K, V>,
}

// Search.

fn get_value<'a, K: Borrow<Q>, V, Q: Ord + ?Sized>(mut node_ref: &'a Tree<K, V>, key: &Q) -> Option<&'a V> {
    while let Some(node) = node_ref {
        match key.cmp(node.key.borrow()) {
            Ordering::Less => node_ref = &node.left,
            Ordering::Equal => return Some(&node.value),
            Ordering::Greater => node_ref = &node.right,
        }
    }

    None
}

// Insertion.

fn left_rotate<K: Ord, V>(node_ref: &mut Tree<K, V>) {
    let mut node = node_ref.take().unwrap();
    let mut right = node.right.take().unwrap();

    node.right = right.left.take();
    right.left = Some(node);

    *node_ref = Some(right);
}

fn right_rotate<K: Ord, V>(node_ref: &mut Tree<K, V>) {
    let mut node = node_ref.take().unwrap();
    let mut left = node.left.take().unwrap();

    node.left = left.right.take();
    left.right = Some(node);

    *node_ref = Some(left);
}

fn relaxed_insert<K: Ord, V>(node_ref: &mut Tree<K, V>, key: K, value: V) -> Result<(), Option<V>> {
    if let Some(node) = node_ref {
        match key.cmp(&node.key) {
            Ordering::Less => {
                if let Some(left) = &mut node.left {
                    match left.color {
                        Color::Red => match key.cmp(&left.key) {
                            Ordering::Less => {
                                relaxed_insert(&mut left.left, key, value)?;

                                left.color = Color::Black;
                                node.color = Color::Red;

                                if let Some(right) = node.right.as_mut().filter(|x| x.color == Color::Red) {
                                    right.color = Color::Black;

                                    Ok(())
                                } else {
                                    right_rotate(node_ref);

                                    Err(None)
                                }
                            }
                            Ordering::Equal => Err(Some(mem::replace(&mut left.value, value))),
                            Ordering::Greater => {
                                relaxed_insert(&mut left.right, key, value)?;

                                node.color = Color::Red;

                                if let Some(right) = node.right.as_mut().filter(|x| x.color == Color::Red) {
                                    left.color = Color::Black;
                                    right.color = Color::Black;

                                    Ok(())
                                } else {
                                    left.right.as_mut().unwrap().color = Color::Black;

                                    left_rotate(&mut node.left);
                                    right_rotate(node_ref);

                                    Ok(())
                                }
                            }
                        },
                        Color::Black => {
                            relaxed_insert(&mut node.left, key, value)?;

                            Err(None)
                        }
                    }
                } else {
                    node.left = Some(Box::new(Node {
                        key,
                        value,
                        color: Color::Red,
                        left: None,
                        right: None,
                    }));

                    Err(None)
                }
            }
            Ordering::Equal => Err(Some(mem::replace(&mut node.value, value))),
            Ordering::Greater => {
                if let Some(right) = &mut node.right {
                    match right.color {
                        Color::Red => match key.cmp(&right.key) {
                            Ordering::Less => {
                                relaxed_insert(&mut right.left, key, value)?;
                                node.color = Color::Red;

                                if let Some(left) = node.left.as_mut().filter(|x| x.color == Color::Red) {
                                    right.color = Color::Black;
                                    left.color = Color::Black;

                                    Ok(())
                                } else {
                                    right.left.as_mut().unwrap().color = Color::Black;

                                    right_rotate(&mut node.right);
                                    left_rotate(node_ref);

                                    Err(None)
                                }
                            }
                            Ordering::Equal => Err(Some(mem::replace(&mut right.value, value))),
                            Ordering::Greater => {
                                relaxed_insert(&mut right.right, key, value)?;

                                right.color = Color::Black;
                                node.color = Color::Red;

                                if let Some(left) = node.left.as_mut().filter(|x| x.color == Color::Red) {
                                    left.color = Color::Black;

                                    Ok(())
                                } else {
                                    left_rotate(node_ref);

                                    Err(None)
                                }
                            }
                        },
                        Color::Black => {
                            relaxed_insert(&mut node.right, key, value)?;

                            Err(None)
                        }
                    }
                } else {
                    node.right = Some(Box::new(Node {
                        key,
                        value,
                        color: Color::Red,
                        right: None,
                        left: None,
                    }));

                    Err(None)
                }
            }
        }
    } else {
        *node_ref = Some(Box::new(Node {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
        }));

        Ok(())
    }
}

fn insert<K: Ord, V>(tree: &mut Tree<K, V>, key: K, value: V) -> Option<V> {
    match relaxed_insert(tree, key, value) {
        Ok(_) => {
            tree.as_mut().unwrap().color = Color::Black;
            None
        }
        Err(maybe_old_value) => maybe_old_value,
    }
}

// Public interface.

pub struct RedBlackTreeMap<K, V> {
    root: Tree<K, V>,
}

impl<K, V> Default for RedBlackTreeMap<K, V> {
    fn default() -> Self {
        RedBlackTreeMap { root: None }
    }
}

impl<K: Ord, V> RedBlackTreeMap<K, V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        get_value(&self.root, key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        insert(&mut self.root, key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::{insert, Color, Node, RedBlackTreeMap, Tree};

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
}
