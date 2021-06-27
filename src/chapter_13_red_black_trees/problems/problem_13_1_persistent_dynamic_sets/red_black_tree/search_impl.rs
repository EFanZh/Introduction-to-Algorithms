use super::{RedBlackTree, RedNode, RedOrBlackNode};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::rc::Rc;

fn persistent_red_black_tree_search_red<'a, K: Borrow<Q>, V, Q: Ord + ?Sized>(
    node: &'a RedNode<K, V>,
    key: &Q,
) -> Option<&'a Rc<V>> {
    match key.cmp((*node.content.key).borrow()) {
        Ordering::Less => persistent_red_black_tree_search(&node.left, key),
        Ordering::Equal => Some(&node.content.value),
        Ordering::Greater => persistent_red_black_tree_search(&node.right, key),
    }
}

fn persistent_red_black_tree_search_red_or_black<'a, K: Borrow<Q>, V, Q: Ord + ?Sized>(
    node: &'a RedOrBlackNode<K, V>,
    key: &Q,
) -> Option<&'a Rc<V>> {
    match node {
        RedOrBlackNode::Red(node) => persistent_red_black_tree_search_red(node, key),
        RedOrBlackNode::Black(node) => persistent_red_black_tree_search(node, key),
    }
}

pub fn persistent_red_black_tree_search<'a, K: Borrow<Q>, V, Q: Ord + ?Sized>(
    tree: &'a RedBlackTree<K, V>,
    key: &Q,
) -> Option<&'a Rc<V>> {
    tree.as_ref()
        .and_then(|node| match key.cmp((*node.content.key).borrow()) {
            Ordering::Less => persistent_red_black_tree_search_red_or_black(&node.left, key),
            Ordering::Equal => Some(&node.content.value),
            Ordering::Greater => persistent_red_black_tree_search_red_or_black(&node.right, key),
        })
}

#[cfg(test)]
mod tests {
    use super::super::tests::{black, black_leaf, red, red_leaf};
    use super::super::{BlackNode, RedBlackTree};
    use super::persistent_red_black_tree_search;
    use std::borrow::Borrow;
    use std::rc::Rc;

    fn search<K: Borrow<Q>, V, T: Into<RedBlackTree<K, V>>, Q: Ord>(tree: T, key: &Q) -> Option<Rc<V>> {
        persistent_red_black_tree_search(&tree.into(), key).cloned()
    }

    #[test]
    fn test_search_not_found() {
        assert_eq!(search(None::<Rc<BlackNode<i32, i32>>>, &4), None);
    }

    #[test]
    fn test_search_red() {
        assert_eq!(search(black(2, 3, red_leaf(1, 2), red_leaf(3, 5)), &1), Some(2.into()));
        assert_eq!(search(black(2, 3, red_leaf(1, 2), red_leaf(3, 5)), &3), Some(5.into()));
    }

    #[test]
    fn test_search_black() {
        assert_eq!(search(black_leaf(2, 3), &2), Some(3.into()));

        let tree_1 = black(2, 3, black_leaf(1, 2), black_leaf(3, 5));

        assert_eq!(search(Rc::clone(&tree_1), &1), Some(2.into()));
        assert_eq!(search(tree_1, &3), Some(5.into()));

        let tree_2 = black(
            4,
            7,
            red(2, 3, black_leaf(1, 2), black_leaf(3, 5)),
            red(6, 13, black_leaf(5, 11), black_leaf(7, 17)),
        );

        assert_eq!(search(Rc::clone(&tree_2), &1), Some(2.into()));
        assert_eq!(search(Rc::clone(&tree_2), &3), Some(5.into()));
        assert_eq!(search(Rc::clone(&tree_2), &5), Some(11.into()));
        assert_eq!(search(tree_2, &7), Some(17.into()));
    }
}
