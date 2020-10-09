use std::cmp::Ordering;
use std::mem;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    priority: usize,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

type Tree<K, V> = Option<Box<Node<K, V>>>;

fn left_rotate<K, V>(node: &mut Box<Node<K, V>>) {
    let mut right = node.right.take().unwrap();

    node.right = right.left.take();
    node.left = Some(mem::replace(node, right));
}

fn right_rotate<K, V>(node: &mut Box<Node<K, V>>) {
    let mut left = node.left.take().unwrap();

    node.left = left.right.take();
    node.right = Some(mem::replace(node, left));
}

fn treap_insert_helper<K: Ord, V>(
    tree: &mut Tree<K, V>,
    key: K,
    value: V,
    priority: usize,
) -> (&mut Node<K, V>, Option<V>) {
    #[allow(clippy::option_if_let_else)]
    if let Some(node) = tree {
        let result = match key.cmp(&node.key) {
            Ordering::Less => {
                let (new_left, result) = treap_insert_helper(&mut node.left, key, value, priority);

                if new_left.priority < node.priority {
                    right_rotate(node);
                }

                result
            }
            Ordering::Equal => Some(mem::replace(&mut node.value, value)),
            Ordering::Greater => {
                let (new_right, result) = treap_insert_helper(&mut node.right, key, value, priority);

                if new_right.priority < node.priority {
                    left_rotate(node);
                }

                result
            }
        };

        (node, result)
    } else {
        *tree = Some(Box::new(Node {
            key,
            value,
            priority,
            left: None,
            right: None,
        }));

        (tree.as_mut().unwrap(), None)
    }
}

pub fn treap_insert<K: Ord, V>(tree: &mut Tree<K, V>, key: K, value: V) -> Option<V> {
    treap_insert_helper(tree, key, value, rand::random()).1
}

#[cfg(test)]
mod tests {
    use super::{treap_insert, treap_insert_helper, Node, Tree};

    trait IntoTree<K, V> {
        fn into_tree(self) -> Tree<K, V>;
    }

    impl<K, V, L: IntoTree<K, V>, R: IntoTree<K, V>> IntoTree<K, V> for (K, V, usize, L, R) {
        fn into_tree(self) -> Tree<K, V> {
            let (key, value, priority, left, right) = self;
            let left = left.into_tree();
            let right = right.into_tree();

            Some(Box::new(Node {
                key,
                value,
                priority,
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

    impl<K, V> IntoTree<K, V> for (K, V, usize) {
        fn into_tree(self) -> Tree<K, V> {
            let (key, value, priority) = self;

            Some(Box::new(Node {
                key,
                value,
                priority,
                left: None,
                right: None,
            }))
        }
    }

    fn insert(mut tree: Tree<i32, i32>, key: i32, value: i32, priority: usize) -> (Tree<i32, i32>, Option<i32>) {
        let result = treap_insert_helper(&mut tree, key, value, priority).1;

        (tree, result)
    }

    #[test]
    fn test_treap_insert_helper_empty() {
        assert_eq!(insert(().into_tree(), 2, 3, 4), ((2, 3, 4).into_tree(), None));
    }

    #[test]
    fn test_treap_insert_helper_found() {
        assert_eq!(
            insert((2, 3, 0, (1, 2, 1), (3, 5, 2)).into_tree(), 1, 7, 77),
            ((2, 3, 0, (1, 7, 1), (3, 5, 2)).into_tree(), Some(2))
        );

        assert_eq!(
            insert((2, 3, 0, (1, 2, 1), (3, 5, 2)).into_tree(), 2, 7, 77),
            ((2, 7, 0, (1, 2, 1), (3, 5, 2)).into_tree(), Some(3))
        );

        assert_eq!(
            insert((2, 3, 0, (1, 2, 1), (3, 5, 2)).into_tree(), 3, 7, 77),
            ((2, 3, 0, (1, 2, 1), (3, 7, 2)).into_tree(), Some(5))
        );
    }

    #[test]
    fn test_treap_insert_helper_left_no_adjust() {
        assert_eq!(
            insert((2, 3, 4).into_tree(), 1, 2, 5),
            ((2, 3, 4, (1, 2, 5), ()).into_tree(), None)
        );
    }

    #[test]
    fn test_treap_insert_helper_left_adjust() {
        assert_eq!(
            insert((2, 3, 4).into_tree(), 1, 2, 3),
            ((1, 2, 3, (), (2, 3, 4)).into_tree(), None)
        );
    }

    #[test]
    fn test_treap_insert_helper_right_no_adjust() {
        assert_eq!(
            insert((2, 3, 4).into_tree(), 3, 5, 5),
            ((2, 3, 4, (), (3, 5, 5)).into_tree(), None)
        );
    }

    #[test]
    fn test_treap_insert_helper_right_adjust() {
        assert_eq!(
            insert((2, 3, 4).into_tree(), 3, 5, 3),
            ((3, 5, 3, (2, 3, 4), ()).into_tree(), None)
        );
    }

    #[test]
    fn test_treap_insert() {
        let mut tree = None;

        treap_insert(&mut tree, 4, 3);

        match tree {
            None => unreachable!(),
            Some(node) => {
                assert_eq!(node.key, 4);
                assert_eq!(node.value, 3);
            }
        }
    }
}
