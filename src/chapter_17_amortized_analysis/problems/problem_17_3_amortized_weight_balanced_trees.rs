use std::borrow::Borrow;
use std::cmp::Ordering;
use std::iter;
use std::mem;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    size: usize,
}

fn get_size<K, V>(maybe_node: &Option<Box<Node<K, V>>>) -> usize {
    maybe_node.as_deref().map_or(0, |node| node.size)
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, left: Option<Box<Self>>, right: Option<Box<Self>>) -> Self {
        let left_size = get_size(&left);
        let right_size = get_size(&right);

        Self {
            key,
            value,
            left,
            right,
            size: left_size + right_size + 1,
        }
    }
}

// Rebalance.

fn into_node_iter<K, V>(mut maybe_node: Option<Box<Node<K, V>>>) -> impl Iterator<Item = Box<Node<K, V>>> {
    let mut stack = Vec::new();

    iter::from_fn(move || loop {
        if let Some(mut node) = maybe_node.take() {
            maybe_node = node.left.take();
            stack.push(node);
        } else if let Some(mut node) = stack.pop() {
            maybe_node = node.right.take();

            return Some(node);
        } else {
            return None;
        }
    })
}

fn rebuild_tree<K, V, I: Iterator<Item = Box<Node<K, V>>>>(iter: &mut I, length: usize) -> Option<Box<Node<K, V>>> {
    if length == 0 {
        None
    } else {
        let half = length / 2;
        let left = rebuild_tree(iter, half);
        let mut middle = iter.next().unwrap();
        let right = rebuild_tree(iter, length - (half + 1));

        middle.size = get_size(&left) + get_size(&right) + 1;
        middle.left = left;
        middle.right = right;

        Some(middle)
    }
}

pub fn rebalance<K, V>(tree: &mut Option<Box<Node<K, V>>>) {
    let length = get_size(tree);

    *tree = rebuild_tree(&mut into_node_iter(tree.take()), length);
}

// Insertion.

fn insert_no_adjust<K: Ord, V>(tree: &mut Option<Box<Node<K, V>>>, key: K, value: V) -> Option<V> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        let result = match key.cmp(&node.key) {
            Ordering::Less => insert_no_adjust(&mut node.left, key, value),
            Ordering::Equal => Some(mem::replace(&mut node.value, value)),
            Ordering::Greater => insert_no_adjust(&mut node.right, key, value),
        };

        if result.is_none() {
            node.size += 1;
        }

        result
    } else {
        *tree = Some(Box::new(Node::new(key, value, None, None)));

        None
    }
}

pub fn insert<K: Ord, V>(tree: &mut Option<Box<Node<K, V>>>, balance_factor: f64, key: K, value: V) -> Option<V> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        match key.cmp(&node.key) {
            Ordering::Less => {
                let max_children = ((node.size + 1) as f64 * balance_factor).floor() as usize;

                if get_size(&node.left) + 1 > max_children {
                    let result = insert_no_adjust(&mut node.left, key, value);

                    if result.is_none() {
                        node.size += 1;

                        rebalance(tree);
                    }

                    result
                } else {
                    let result = insert(&mut node.left, balance_factor, key, value);

                    if result.is_none() {
                        node.size += 1;
                    }

                    result
                }
            }
            Ordering::Equal => Some(mem::replace(&mut node.value, value)),
            Ordering::Greater => {
                let max_children = ((node.size + 1) as f64 * balance_factor).floor() as usize;

                if get_size(&node.right) + 1 > max_children {
                    let result = insert_no_adjust(&mut node.right, key, value);

                    if result.is_none() {
                        node.size += 1;

                        rebalance(tree);
                    }

                    result
                } else {
                    let result = insert(&mut node.right, balance_factor, key, value);

                    if result.is_none() {
                        node.size += 1;
                    }

                    result
                }
            }
        }
    } else {
        *tree = Some(Box::new(Node::new(key, value, None, None)));

        None
    }
}

// Removal.

fn extract_min_no_adjust<K, V>(node_ref: &mut Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = node_ref {
        Some(if let Some(result) = extract_min_no_adjust(&mut node.left) {
            node.size -= 1;

            result
        } else {
            let right = node.right.take();

            node.size = 1;

            mem::replace(node_ref, right).unwrap()
        })
    } else {
        None
    }
}

fn extract_min<K, V>(node_ref: &mut Option<Box<Node<K, V>>>, balance_factor: f64) -> Option<Box<Node<K, V>>> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = node_ref {
        let max_children = ((node.size - 1) as f64 * balance_factor).floor() as usize;

        Some(if get_size(&node.right) > max_children {
            if let Some(result) = extract_min_no_adjust(&mut node.left) {
                node.size -= 1;

                rebalance(node_ref);

                result
            } else {
                let right = node.right.take();

                node.size = 1;

                mem::replace(node_ref, right).unwrap()
            }
        } else if let Some(result) = extract_min(&mut node.left, balance_factor) {
            node.size -= 1;

            result
        } else {
            let right = node.right.take();

            node.size = 1;

            mem::replace(node_ref, right).unwrap()
        })
    } else {
        None
    }
}

fn lift_min_no_adjust<K, V>(node: &mut Box<Node<K, V>>) {
    if let Some(min) = extract_min_no_adjust(&mut node.left) {
        node.size -= 1;

        let old_node = mem::replace(node, min);

        node.size += old_node.size;
        node.right = Some(old_node);
    }
}

fn lift_min<K, V>(node: &mut Box<Node<K, V>>, balance_factor: f64) {
    let max_children = ((node.size - 1) as f64 * balance_factor).floor() as usize;

    if get_size(&node.right) > max_children {
        if let Some(min) = extract_min_no_adjust(&mut node.left) {
            node.size -= 1;

            let old_node = mem::replace(node, min);

            node.size += old_node.size;
            node.right = Some(old_node);

            rebalance(&mut node.right);
        }
    } else if let Some(min) = extract_min(&mut node.left, balance_factor) {
        node.size -= 1;

        let old_node = mem::replace(node, min);

        node.size += old_node.size;
        node.right = Some(old_node);
    }
}

fn remove_root_no_adjust<K: Borrow<Q>, V, Q: Ord + ?Sized>(root: &mut Option<Box<Node<K, V>>>) -> V {
    let node = root.as_mut().unwrap();

    match (node.left.take(), node.right.take()) {
        (None, None) => root.take().unwrap(),
        (None, Some(child)) | (Some(child), None) => mem::replace(node, child),
        (Some(left), Some(mut right)) => {
            lift_min_no_adjust(&mut right);

            right.size += left.size;
            right.left = Some(left);

            mem::replace(node, right)
        }
    }
    .value
}

fn remove_root<K: Borrow<Q>, V, Q: Ord + ?Sized>(root: &mut Option<Box<Node<K, V>>>, balance_factor: f64) -> V {
    let node = root.as_mut().unwrap();

    match (node.left.take(), node.right.take()) {
        (None, None) => root.take().unwrap(),
        (None, Some(child)) | (Some(child), None) => mem::replace(node, child),
        (Some(left), Some(mut right)) => {
            lift_min(&mut right, balance_factor);

            right.size += left.size;
            right.left = Some(left);

            mem::replace(node, right)
        }
    }
    .value
}

fn remove_no_adjust<K: Borrow<Q>, V, Q: Ord + ?Sized>(tree: &mut Option<Box<Node<K, V>>>, key: &Q) -> Option<V> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        match key.cmp(node.key.borrow()) {
            Ordering::Less => remove_no_adjust(&mut node.left, key),
            Ordering::Equal => Some(remove_root_no_adjust(tree)),
            Ordering::Greater => remove_no_adjust(&mut node.right, key),
        }
    } else {
        None
    }
}

pub fn remove<K: Borrow<Q>, V, Q: Ord + ?Sized>(
    tree: &mut Option<Box<Node<K, V>>>,
    balance_factor: f64,
    key: &Q,
) -> Option<V> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        let max_children = ((node.size - 1) as f64 * balance_factor).floor() as usize;

        match key.cmp(node.key.borrow()) {
            Ordering::Less => {
                if get_size(&node.right) > max_children {
                    let result = remove_no_adjust(&mut node.left, key);

                    if result.is_some() {
                        node.size -= 1;
                        rebalance(tree);
                    }

                    result
                } else {
                    let result = remove(&mut node.left, balance_factor, key);

                    if result.is_some() {
                        node.size -= 1;
                    }

                    result
                }
            }
            Ordering::Equal => Some(if get_size(&node.left) > max_children {
                let result = remove_root_no_adjust(tree);

                rebalance(tree);

                result
            } else {
                remove_root(tree, balance_factor)
            }),
            Ordering::Greater => {
                if get_size(&node.left) > max_children {
                    let result = remove_no_adjust(&mut node.right, key);

                    if result.is_some() {
                        node.size -= 1;
                        rebalance(tree);
                    }

                    result
                } else {
                    let result = remove(&mut node.right, balance_factor, key);

                    if result.is_some() {
                        node.size -= 1;
                    }

                    result
                }
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{insert, insert_no_adjust, into_node_iter, rebalance, remove, remove_no_adjust, Node};

    macro_rules! make_tree {
        () => {
            None
        };
        ($key:expr, $value:expr) => {
            make_tree!($key, $value, (), ())
        };
        ($key:expr, $value:expr, ($($left:tt)*), ($($right:tt)*)) => {
            Some(Box::new(Node::new($key, $value, make_tree!($($left)*), make_tree!($($right)*))))
        };
    }

    #[test]
    fn test_into_node_iter() {
        let test_cases = [
            (make_tree!(), &[] as &[_]),
            (make_tree!(1, 2), &[(1, 2)]),
            (make_tree!(3, 4, (1, 2), ()), &[(1, 2), (3, 4)]),
            (make_tree!(1, 2, (), (3, 4)), &[(1, 2), (3, 4)]),
            (make_tree!(3, 4, (1, 2), (5, 6)), &[(1, 2), (3, 4), (5, 6)]),
        ];

        for (tree, expected) in test_cases.iter().cloned() {
            assert_eq!(
                into_node_iter(tree)
                    .map(|node: Box<Node<i32, i32>>| (node.key, node.value))
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }

    #[test]
    fn test_rebalance() {
        let test_cases = [
            (
                make_tree!(5, 55, (4, 44, (3, 33, (2, 22, (1, 11, (0, -17), ()), ()), ()), ()), ()),
                make_tree!(3, 33, (1, 11, (0, -17), (2, 22)), (5, 55, (4, 44), ())),
            ),
            (
                make_tree!(0, -17, (), (1, 11, (), (2, 22, (), (3, 33, (), (4, 44, (), (5, 55)))))),
                make_tree!(3, 33, (1, 11, (0, -17), (2, 22)), (5, 55, (4, 44), ())),
            ),
        ];

        for (mut tree, expcted) in test_cases.iter().cloned() {
            rebalance(&mut tree);

            assert_eq!(tree, expcted);
        }
    }

    #[test]
    fn test_insert_no_adjust() {
        let test_cases = [
            ((make_tree!(), 2, 3), (None, make_tree!(2, 3))),
            ((make_tree!(2, 3), 1, 4), (None, make_tree!(2, 3, (1, 4), ()))),
            ((make_tree!(2, 3), 2, 4), (Some(3), make_tree!(2, 4))),
            ((make_tree!(2, 3), 3, 4), (None, make_tree!(2, 3, (), (3, 4)))),
        ];

        for ((mut tree, key, value), (expected_value, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(insert_no_adjust(&mut tree, key, value), expected_value);
            assert_eq!(tree, expected_tree);
        }
    }

    #[test]
    fn test_insert() {
        let test_cases = [
            ((make_tree!(), 0.5, 2, 3), (None, make_tree!(2, 3))),
            ((make_tree!(2, 3), 0.5, 1, 4), (None, make_tree!(2, 3, (1, 4), ()))),
            (
                (make_tree!(2, 3, (1, 4), ()), 0.5, 0, 5),
                (None, make_tree!(1, 4, (0, 5), (2, 3))),
            ),
            (
                (make_tree!(2, 3, (1, 4), ()), 0.5, 1, 5),
                (Some(4), make_tree!(2, 3, (1, 5), ())),
            ),
            ((make_tree!(2, 3), 0.5, 2, 4), (Some(3), make_tree!(2, 4))),
            ((make_tree!(2, 3), 0.5, 3, 4), (None, make_tree!(2, 3, (), (3, 4)))),
            (
                (make_tree!(2, 3, (), (3, 4)), 0.5, 4, 5),
                (None, make_tree!(3, 4, (2, 3), (4, 5))),
            ),
            (
                (make_tree!(2, 3, (), (3, 4)), 0.5, 3, 5),
                (Some(4), make_tree!(2, 3, (), (3, 5))),
            ),
        ];

        for ((mut tree, balance_factor, key, value), (expected_value, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(insert(&mut tree, balance_factor, key, value), expected_value);
            assert_eq!(tree, expected_tree);
        }
    }

    #[test]
    fn test_remove_no_adjust() {
        let test_cases = [
            ((make_tree!(), 3), (None, make_tree!())),
            ((make_tree!(2, 3), 4), (None, make_tree!(2, 3))),
            ((make_tree!(4, 6), 4), (Some(6), make_tree!())),
            ((make_tree!(2, 3, (), (4, 5)), 2), (Some(3), make_tree!(4, 5))),
            (
                (make_tree!(2, 3, (1, 2), (4, 5)), 2),
                (Some(3), make_tree!(4, 5, (1, 2), ())),
            ),
            (
                (make_tree!(2, 3, (1, 2), (4, 5, (3, 7), ())), 2),
                (Some(3), make_tree!(3, 7, (1, 2), (4, 5))),
            ),
            (
                (make_tree!(2, 3, (1, 2), (5, 8, (4, 6, (3, 5), ()), ())), 2),
                (Some(3), make_tree!(3, 5, (1, 2), (5, 8, (4, 6), ()))),
            ),
            (
                (make_tree!(2, 3, (1, 2), (3, 5)), 2),
                (Some(3), make_tree!(3, 5, (1, 2), ())),
            ),
        ];

        for ((mut tree, key), (expected_value, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(remove_no_adjust(&mut tree, &key), expected_value);
            assert_eq!(tree, expected_tree);
        }
    }

    #[test]
    fn test_remove_part_1() {
        let test_cases = [
            ((make_tree!(), 0.5, 3), (None, make_tree!())),
            ((make_tree!(2, 3), 0.5, 4), (None, make_tree!(2, 3))),
            ((make_tree!(4, 6), 0.5, 4), (Some(6), make_tree!())),
            (
                (make_tree!(3, 5, (2, 3), (5, 7, (), (11, 13))), 0.5, 2),
                (Some(3), make_tree!(5, 7, (3, 5), (11, 13))),
            ),
            (
                (make_tree!(3, 5, (2, 3, (1, 7), ()), (5, 7)), 0.5, 2),
                (Some(3), make_tree!(3, 5, (1, 7), (5, 7))),
            ),
            (
                (make_tree!(3, 5, (2, 3), (5, 7, (), (11, 13))), 0.5, 5),
                (Some(7), make_tree!(3, 5, (2, 3), (11, 13))),
            ),
            (
                (make_tree!(3, 5, (2, 3, (1, 7), ()), (5, 7)), 0.5, 5),
                (Some(7), make_tree!(2, 3, (1, 7), (3, 5))),
            ),
            (
                (make_tree!(3, 5, (2, 3, (1, 2), ()), (7, 11)), 0.5, 3),
                (Some(5), make_tree!(2, 3, (1, 2), (7, 11))),
            ),
            (
                (make_tree!(3, 5, (2, 3), (7, 11)), 0.5, 3),
                (Some(5), make_tree!(7, 11, (2, 3), ())),
            ),
            (
                (
                    make_tree!(7, 4, (3, 6, (2, 1), (5, 8)), (13, 2, (11, 6), (19, 5))),
                    0.5,
                    7,
                ),
                (Some(4), make_tree!(11, 6, (3, 6, (2, 1), (5, 8)), (13, 2, (), (19, 5)))),
            ),
            (
                (
                    make_tree!(7, 4, (3, 6, (2, 1), (5, 8)), (13, 2, (11, 6), (19, 5, (17, 7), ()))),
                    0.5,
                    7,
                ),
                (
                    Some(4),
                    make_tree!(11, 6, (3, 6, (2, 1), (5, 8)), (17, 7, (13, 2), (19, 5))),
                ),
            ),
        ];

        for ((mut tree, balabce_factor, key), (expected_value, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(remove(&mut tree, balabce_factor, &key), expected_value);
            assert_eq!(tree, expected_tree);
        }
    }

    #[test]
    fn test_remove_part_2() {
        let test_cases = [
            (
                (
                    make_tree!(
                        8,
                        4,
                        (4, 1, (2, 6, (1, 5), (3, 8)), (6, 6, (5, 7), (7, 9))),
                        (13, 3, (10, 9, (9, 2), (12, 3, (11, 6), ())), (15, 8, (14, 2), (16, 1)))
                    ),
                    0.5,
                    8,
                ),
                (
                    Some(4),
                    make_tree!(
                        9,
                        2,
                        (4, 1, (2, 6, (1, 5), (3, 8)), (6, 6, (5, 7), (7, 9))),
                        (13, 3, (11, 6, (10, 9), (12, 3)), (15, 8, (14, 2), (16, 1)))
                    ),
                ),
            ),
            (
                (
                    make_tree!(
                        8,
                        4,
                        (4, 1, (2, 6, (1, 5), (3, 8)), (6, 6, (5, 7), (7, 9))),
                        (13, 3, (10, 9, (9, 2), (12, 3)), (15, 8, (14, 2), (16, 1)))
                    ),
                    0.5,
                    8,
                ),
                (
                    Some(4),
                    make_tree!(
                        9,
                        2,
                        (4, 1, (2, 6, (1, 5), (3, 8)), (6, 6, (5, 7), (7, 9))),
                        (13, 3, (10, 9, (), (12, 3)), (15, 8, (14, 2), (16, 1)))
                    ),
                ),
            ),
            (
                (
                    make_tree!(7, 9, (3, 6, (2, 3), (5, 8)), (15, 18, (11, 6, (), (13, 2)), (17, 4))),
                    0.5,
                    7,
                ),
                (
                    Some(9),
                    make_tree!(11, 6, (3, 6, (2, 3), (5, 8)), (15, 18, (13, 2), (17, 4))),
                ),
            ),
        ];

        for ((mut tree, balabce_factor, key), (expected_value, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(remove(&mut tree, balabce_factor, &key), expected_value);
            assert_eq!(tree, expected_tree);
        }
    }
}
