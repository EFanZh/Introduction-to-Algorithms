use std::cmp::Ordering;
use std::mem;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    height: i32,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

type Tree<K, V> = Option<Box<Node<K, V>>>;

fn get_height<K, V>(node: &Option<Box<Node<K, V>>>) -> i32 {
    node.as_ref().map_or(0, |x| x.height)
}

fn get_balance_factor<K, V>(node: &Node<K, V>) -> i32 {
    get_height(&node.left) - get_height(&node.right)
}

fn fix_height<K, V>(node: &mut Node<K, V>) -> bool {
    let expected_height = get_height(&node.left).max(get_height(&node.right)) + 1;
    let needs_fixing = node.height != expected_height;

    node.height = expected_height;

    needs_fixing
}

fn left_rotate<K, V>(node: &mut Box<Node<K, V>>) {
    let mut right = node.right.take().unwrap();

    node.right = right.left.take();

    fix_height(node);

    node.left = Some(mem::replace(node, right));

    fix_height(node);
}

fn right_rotate<K, V>(node: &mut Box<Node<K, V>>) {
    let mut left = node.left.take().unwrap();

    node.left = left.right.take();

    fix_height(node);

    node.right = Some(mem::replace(node, left));

    fix_height(node);
}

fn make_left_higher<K, V>(node: &mut Box<Node<K, V>>) {
    if get_balance_factor(node) == -1 {
        left_rotate(node);
    }
}

fn make_right_higher<K, V>(node: &mut Box<Node<K, V>>) {
    if get_balance_factor(node) == 1 {
        right_rotate(node);
    }
}

fn balance<K, V>(node: &mut Box<Node<K, V>>) {
    match get_balance_factor(node) {
        -2 => {
            make_right_higher(node.right.as_mut().unwrap());
            left_rotate(node);
        }
        2 => {
            make_left_higher(node.left.as_mut().unwrap());
            right_rotate(node);
        }
        _ => (),
    }
}

pub fn avl_insert<K: Ord, V>(tree: &mut Tree<K, V>, key: K, value: V) -> Option<V> {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        match key.cmp(&node.key) {
            Ordering::Less => {
                let result = avl_insert(&mut node.left, key, value);

                if fix_height(node) {
                    balance(node);
                }

                result
            }
            Ordering::Equal => Some(mem::replace(&mut node.value, value)),
            Ordering::Greater => {
                let result = avl_insert(&mut node.right, key, value);

                if fix_height(node) {
                    balance(node);
                }

                result
            }
        }
    } else {
        *tree = Some(Box::new(Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }));

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{avl_insert, get_height, Node, Tree};

    trait IntoTree<K, V> {
        fn into_tree(self) -> Tree<K, V>;
    }

    impl<K, V, L: IntoTree<K, V>, R: IntoTree<K, V>> IntoTree<K, V> for (K, V, L, R) {
        fn into_tree(self) -> Tree<K, V> {
            let (key, value, left, right) = self;
            let left = left.into_tree();
            let right = right.into_tree();
            let height = get_height(&left).max(get_height(&right)) + 1;

            Some(Box::new(Node {
                key,
                value,
                height,
                left,
                right,
            }))
        }
    }

    impl<K, V> IntoTree<K, V> for () {
        fn into_tree(self) -> Tree<K, V> {
            None
        }
    }

    impl<K, V> IntoTree<K, V> for (K, V) {
        fn into_tree(self) -> Tree<K, V> {
            let (key, value) = self;

            Some(Box::new(Node {
                key,
                value,
                height: 1,
                left: None,
                right: None,
            }))
        }
    }

    fn insert(mut tree: Tree<i32, i32>, key: i32, value: i32) -> (Tree<i32, i32>, Option<i32>) {
        let result = avl_insert(&mut tree, key, value);

        (tree, result)
    }

    #[test]
    fn test_insert_left_left() {
        assert_eq!(
            insert((4, 3, (2, 2), ()).into_tree(), 1, 7),
            ((2, 2, (1, 7), (4, 3)).into_tree(), None)
        );

        assert_eq!(
            insert((6, 13, (4, 7, (2, 3), ()), (8, 19)).into_tree(), 1, 7),
            ((6, 13, (2, 3, (1, 7), (4, 7)), (8, 19)).into_tree(), None)
        );
    }

    #[test]
    fn test_insert_left_right() {
        assert_eq!(
            insert((4, 3, (2, 2), ()).into_tree(), 3, 7),
            ((3, 7, (2, 2), (4, 3)).into_tree(), None)
        );

        assert_eq!(
            insert((6, 13, (4, 7, (2, 3), ()), (8, 19)).into_tree(), 3, 7),
            ((6, 13, (3, 7, (2, 3), (4, 7)), (8, 19)).into_tree(), None),
        );
    }

    #[test]
    fn test_insert_right_left() {
        assert_eq!(
            insert((1, 3, (), (3, 2)).into_tree(), 4, 7),
            ((3, 2, (1, 3), (4, 7)).into_tree(), None)
        );

        assert_eq!(
            insert((3, 13, (1, 19), (5, 7, (), (7, 3))).into_tree(), 8, 7),
            ((3, 13, (1, 19), (7, 3, (5, 7), (8, 7))).into_tree(), None)
        );
    }

    #[test]
    fn test_insert_right_right() {
        assert_eq!(
            insert((3, 3, (), (5, 2)).into_tree(), 4, 7),
            ((4, 7, (3, 3), (5, 2)).into_tree(), None)
        );

        assert_eq!(
            insert((5, 13, (3, 19), (7, 7, (), (9, 3))).into_tree(), 8, 7),
            ((5, 13, (3, 19), (8, 7, (7, 7), (9, 3))).into_tree(), None),
        );
    }

    #[test]
    fn test_insert_replace() {
        assert_eq!(
            insert((2, 3, (1, 2), (3, 5)).into_tree(), 1, 7),
            ((2, 3, (1, 7), (3, 5)).into_tree(), Some(2))
        );

        assert_eq!(
            insert((2, 3, (1, 2), (3, 5)).into_tree(), 2, 7),
            ((2, 7, (1, 2), (3, 5)).into_tree(), Some(3))
        );

        assert_eq!(
            insert((2, 3, (1, 2), (3, 5)).into_tree(), 3, 7),
            ((2, 3, (1, 2), (3, 7)).into_tree(), Some(5))
        );
    }
}
