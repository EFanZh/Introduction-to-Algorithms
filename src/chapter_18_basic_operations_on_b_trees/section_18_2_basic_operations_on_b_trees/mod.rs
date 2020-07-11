// B-Tree-Search(x, k)
//
// 1  i = 1
// 2  while i ≤ x.n and k > x.key_i
// 3      i = i + 1
// 4  if i ≤ x.n and k == x.key_i
// 5      return (x, i)
// 6  elseif x.leaf
// 7      return nil
// 8  else Disk-Read(x.c_i)
// 9      return B-Tree-Search(x.c_i, k)

use super::section_18_1_definition_of_b_trees::{Node, NodeImpl};
use std::borrow::Borrow;

pub fn b_tree_search<'a, K: Ord + Borrow<Q>, V, Q: Ord + ?Sized>(x: &'a Node<K, V>, k: &Q) -> Option<&'a V> {
    match &x.0 {
        NodeImpl::Internal(node) => {
            let node = node.as_ref();

            match node.data.binary_search_by(|item| item.key.borrow().cmp(k)) {
                Ok(i) => Some(&node.data[i].value),
                Err(i) => {
                    if let Some(item) = node.data.get(i) {
                        b_tree_search(&item.child, k)
                    } else {
                        b_tree_search(&node.last_child, k)
                    }
                }
            }
        }
        NodeImpl::Leaf(node) => {
            let node = node.as_ref();

            match node.data.binary_search_by(|item| item.0.borrow().cmp(k)) {
                Ok(i) => Some(&node.data[i].1),
                Err(_) => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::section_18_1_definition_of_b_trees::tests::make_node;
    use super::b_tree_search;

    #[test]
    fn test_b_tree_search() {
        let test_cases = [
            ((make_node!(), 2), None),
            ((make_node!(1 => 3), 2), None),
            ((make_node!(2 => 3), 2), Some(3)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 1), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 2), Some(3)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 3), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 5), Some(7)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 6), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 11), Some(13)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 12), None),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 1),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 2),
                Some(3),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 3),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 5),
                Some(7),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 6),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 11),
                Some(13),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 12),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 17),
                Some(19),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 18),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 23),
                Some(27),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 24),
                None,
            ),
        ];

        for ((tree, key), expected) in test_cases.iter().cloned() {
            assert_eq!(b_tree_search(&tree, &key).copied(), expected)
        }
    }
}
