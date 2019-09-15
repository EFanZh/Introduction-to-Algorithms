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

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[allow(clippy::borrowed_box)]
fn left_rotate<K, V>(root: &mut Box<Node<K, V>>) {
    let mut right = root.right.take().unwrap();

    root.right = right.left.take();
    root.left = Some(mem::replace(root, right));
}

#[allow(clippy::borrowed_box)]
fn right_rotate<K, V>(root: &mut Box<Node<K, V>>) {
    let mut left = root.left.take().unwrap();

    root.left = left.right.take();
    root.right = Some(mem::replace(root, left));
}

#[allow(clippy::borrowed_box)]
fn relaxed_insert_non_null<K: Ord, V>(
    node: &mut Box<Node<K, V>>,
    key: K,
    value: V,
) -> Result<&mut Node<K, V>, Option<V>> {
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

                                Ok(node)
                            } else {
                                right_rotate(node);

                                Err(None)
                            }
                        }
                        Ordering::Equal => Err(Some(mem::replace(&mut left.value, value))),
                        Ordering::Greater => {
                            let left_right = relaxed_insert(&mut left.right, key, value)?;

                            node.color = Color::Red;

                            if let Some(right) = node.right.as_mut().filter(|x| x.color == Color::Red) {
                                left.color = Color::Black;
                                right.color = Color::Black;

                                Ok(node)
                            } else {
                                left_right.color = Color::Black;

                                left_rotate(left);
                                right_rotate(node);

                                Err(None)
                            }
                        }
                    },
                    Color::Black => relaxed_insert_non_null(left, key, value).and(Err(None)),
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
                            let right_left = relaxed_insert(&mut right.left, key, value)?;

                            node.color = Color::Red;

                            if let Some(left) = node.left.as_mut().filter(|x| x.color == Color::Red) {
                                right.color = Color::Black;
                                left.color = Color::Black;

                                Ok(node)
                            } else {
                                right_left.color = Color::Black;

                                right_rotate(right);
                                left_rotate(node);

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

                                Ok(node)
                            } else {
                                left_rotate(node);

                                Err(None)
                            }
                        }
                    },
                    Color::Black => relaxed_insert_non_null(right, key, value).and(Err(None)),
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
}

fn relaxed_insert<K: Ord, V>(node_ref: &mut Tree<K, V>, key: K, value: V) -> Result<&mut Node<K, V>, Option<V>> {
    if let Some(node) = node_ref {
        relaxed_insert_non_null(node, key, value)
    } else {
        *node_ref = Some(Box::new(Node {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
        }));

        Ok(node_ref.as_mut().unwrap())
    }
}

fn insert<K: Ord, V>(tree: &mut Tree<K, V>, key: K, value: V) -> Option<V> {
    match relaxed_insert(tree, key, value) {
        Ok(root) => {
            root.color = Color::Black;

            None
        }
        Err(maybe_old_value) => maybe_old_value,
    }
}

// Deletion.

#[allow(clippy::borrowed_box)]
fn adjust_on_left_child_black_sibling_red_root<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    let right = node.right.as_mut().unwrap();

    if let Some(right_right) = right.right.as_mut().filter(|x| x.color == Color::Red) {
        node.color = Color::Black;
        right.color = Color::Red;
        right_right.color = Color::Black;

        left_rotate(node);
    } else if right.left.as_ref().map(|x| x.color) == Some(Color::Red) {
        node.color = Color::Black;

        right_rotate(right);
        left_rotate(node);
    } else {
        node.color = Color::Black;
        right.color = Color::Red;
    }

    false
}

#[allow(clippy::borrowed_box)]
fn adjust_on_left_child_black_sibling<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    let right = node.right.as_mut().unwrap();

    if let Some(right_right) = right.right.as_mut().filter(|x| x.color == Color::Red) {
        right.color = mem::replace(&mut node.color, Color::Black);
        right_right.color = Color::Red;

        left_rotate(node);
    } else if let Some(right_left) = right.left.as_mut().filter(|x| x.color == Color::Red) {
        right_left.color = mem::replace(&mut node.color, Color::Black);

        right_rotate(right);
        left_rotate(node);
    } else {
        right.color = Color::Red;

        match node.color {
            Color::Red => node.color = Color::Black,
            Color::Black => return true,
        }
    }

    false
}

#[allow(clippy::borrowed_box)]
fn adjust_on_left_child<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    // Change right neighbor into a black node.

    let right = node.right.as_mut().unwrap();

    if right.color == Color::Red {
        node.color = Color::Red;
        right.color = Color::Black;

        left_rotate(node);

        adjust_on_left_child_black_sibling_red_root(node.left.as_mut().unwrap())
    } else {
        adjust_on_left_child_black_sibling(node)
    }
}

#[allow(clippy::borrowed_box)]
fn adjust_on_right_child_black_sibling_red_root<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    let left = node.left.as_mut().unwrap();

    if let Some(left_left) = left.left.as_mut().filter(|x| x.color == Color::Red) {
        node.color = Color::Black;
        left.color = Color::Red;
        left_left.color = Color::Black;

        right_rotate(node);
    } else if left.right.as_ref().map(|x| x.color) == Some(Color::Red) {
        node.color = Color::Black;

        left_rotate(left);
        right_rotate(node);
    } else {
        node.color = Color::Black;
        left.color = Color::Red;
    }

    false
}

#[allow(clippy::borrowed_box)]
fn adjust_on_right_child_black_sibling<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    let left = node.left.as_mut().unwrap();

    if let Some(left_left) = left.left.as_mut().filter(|x| x.color == Color::Red) {
        left.color = mem::replace(&mut node.color, Color::Black);
        left_left.color = Color::Red;

        right_rotate(node);
    } else if let Some(left_right) = left.right.as_mut().filter(|x| x.color == Color::Red) {
        left_right.color = mem::replace(&mut node.color, Color::Black);

        left_rotate(left);
        right_rotate(node);
    } else {
        left.color = Color::Red;

        match node.color {
            Color::Red => node.color = Color::Black,
            Color::Black => return true,
        }
    }

    false
}

#[allow(clippy::borrowed_box)]
fn adjust_on_right_child<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    // Change left neighbor into a black node.

    let left = node.left.as_mut().unwrap();

    if left.color == Color::Red {
        node.color = Color::Red;
        left.color = Color::Black;

        right_rotate(node);

        adjust_on_right_child_black_sibling_red_root(node.right.as_mut().unwrap())
    } else {
        adjust_on_right_child_black_sibling(node)
    }
}

fn remove_min<K, V>(node_ref: &mut Option<Box<Node<K, V>>>) -> (bool, Box<Node<K, V>>) {
    let node = node_ref.as_mut().unwrap();

    if node.left.is_some() {
        let (mut height_changed, min) = remove_min(&mut node.left);

        if height_changed {
            height_changed = adjust_on_left_child(node);
        }

        (height_changed, min)
    } else {
        let min_right = node.right.take();
        let min = mem::replace(node_ref, min_right).unwrap();

        (min.color == Color::Black, min)
    }
}

#[allow(clippy::borrowed_box)]
fn lift_min<K, V>(node: &mut Box<Node<K, V>>) -> bool {
    if node.left.is_some() {
        let (height_changed, min) = remove_min(&mut node.left);

        node.right = Some(mem::replace(node, min));

        height_changed
    } else {
        node.color == Color::Black
    }
}

fn delete<K, V>(node_ref: &mut Tree<K, V>) -> (bool, V) {
    let mut node = node_ref.take().unwrap();

    match (node.left.take(), node.right.take()) {
        (None, None) => (node.color == Color::Black, node.value),
        (None, Some(mut right)) => {
            right.color = Color::Black;
            *node_ref = Some(right);

            (false, node.value)
        }
        (Some(mut left), None) => {
            left.color = Color::Black;
            *node_ref = Some(left);

            (false, node.value)
        }
        (Some(left), Some(mut right)) => {
            let mut height_changed = lift_min(&mut right);

            right.color = node.color;
            right.left = Some(left);

            if height_changed {
                height_changed = adjust_on_right_child(&mut right);
            }

            *node_ref = Some(right);

            (height_changed, node.value)
        }
    }
}

fn remove<K: Borrow<Q>, V, Q: Ord + ?Sized>(node_ref: &mut Tree<K, V>, key: &Q) -> Result<V, Option<V>> {
    if let Some(node) = node_ref {
        match key.cmp(node.key.borrow()) {
            Ordering::Less => {
                let result = remove(&mut node.left, key)?;

                if adjust_on_left_child(node) {
                    Err(Some(result))
                } else {
                    Ok(result)
                }
            }
            Ordering::Equal => {
                let (height_changed, value) = delete(node_ref);

                if height_changed {
                    Ok(value)
                } else {
                    Err(Some(value))
                }
            }
            Ordering::Greater => {
                let result = remove(&mut node.right, key)?;

                if adjust_on_right_child(node) {
                    Err(Some(result))
                } else {
                    Ok(result)
                }
            }
        }
    } else {
        Err(None)
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

    pub fn remove<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        match remove(&mut self.root, key) {
            Ok(result) => Some(result),
            Err(result) => result,
        }
    }
}

#[cfg(test)]
mod tests {
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
}
