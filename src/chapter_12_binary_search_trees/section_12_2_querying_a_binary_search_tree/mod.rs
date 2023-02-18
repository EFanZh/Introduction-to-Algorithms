use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::{
    BinaryTreeNode, SimpleBinaryTreeNode,
};
use std::cmp::Ordering;
use std::rc::Rc;

pub mod exercises;

// Tree-Search(x, k)
//
// 1  if x == nil or k == x.key
// 2      return x
// 3  if k < x.key
// 4      return Tree-Search(x.left, k)
// 5  else return Tree-Search(x.right, k)

pub fn tree_search<'a, T: Ord>(
    x: &'a Option<Box<SimpleBinaryTreeNode<T>>>,
    k: &T,
) -> Option<&'a SimpleBinaryTreeNode<T>> {
    x.as_deref().and_then(|node| match node.key.cmp(k) {
        Ordering::Less => tree_search(&node.right, k),
        Ordering::Equal => Some(node),
        Ordering::Greater => tree_search(&node.left, k),
    })
}

// Iterative-Tree-Search(x, k)
//
// 1  while x ≠ nil and k ≠ x.key
// 2      if k < x.key
// 3          x = x.left
// 4      else x = x.right
// 5  return x

pub fn iterative_tree_search<'a, T: Ord>(
    mut x: &'a Option<Box<SimpleBinaryTreeNode<T>>>,
    k: &T,
) -> Option<&'a SimpleBinaryTreeNode<T>> {
    while let Some(node) = x {
        match node.key.cmp(k) {
            Ordering::Less => x = &node.right,
            Ordering::Equal => return Some(node),
            Ordering::Greater => x = &node.left,
        }
    }

    None
}

// Tree-Minimum(x)
//
// 1  while x.left ≠ nil
// 2      x = x.left
// 3  return x

pub fn tree_minimum<T: Ord>(mut x: &SimpleBinaryTreeNode<T>) -> &SimpleBinaryTreeNode<T> {
    while let Some(left) = &x.left {
        x = left;
    }

    x
}

// Tree-Maximum(x)
//
// 1  while x.right ≠ nil
// 2      x = x.right
// 3      return x

pub fn tree_maximum<T: Ord>(mut x: &SimpleBinaryTreeNode<T>) -> &SimpleBinaryTreeNode<T> {
    while let Some(right) = &x.right {
        x = right;
    }

    x
}

// Tree-Successor(x)
//
// 1  if x.right ≠ nil
// 2      return Tree-Minimum(x.right)
// 3  y = x.p
// 4  while y ≠ nil and x == y.right
// 5      x = y
// 6      y = y.p
// 7  return y

#[allow(clippy::option_if_let_else)] // False positive.
pub fn tree_successor<T: Ord>(mut x: Rc<BinaryTreeNode<T>>) -> Option<Rc<BinaryTreeNode<T>>> {
    fn tree_minimum<T: Ord>(mut x: &Rc<BinaryTreeNode<T>>) -> &Rc<BinaryTreeNode<T>> {
        while let Some(left) = x.get_left_child() {
            x = left;
        }

        x
    }

    if let Some(right) = x.get_right_child() {
        Some(Rc::clone(tree_minimum(right)))
    } else {
        let mut y = x.get_parent();

        while let Some(y_rc) = &y {
            if let Some(y_right_child) = y_rc.get_right_child() {
                if Rc::ptr_eq(&x, y_right_child) {
                    x = Rc::clone(y_rc);
                    y = y_rc.get_parent();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        y
    }
}

#[cfg(test)]
mod tests {
    use crate::{make_simple_tree, make_tree};

    #[test]
    fn test_tree_search() {
        assert!(super::tree_search(&None, &4).is_none());

        let tree_1 = &make_simple_tree![1];

        assert!(super::tree_search(tree_1, &0).is_none());
        assert_eq!(super::tree_search(tree_1, &1).unwrap().key, 1);
        assert!(super::tree_search(tree_1, &4).is_none());

        let tree_2 = &make_simple_tree![(5, 2, 8)];

        assert!(super::tree_search(tree_2, &1).is_none());
        assert_eq!(super::tree_search(tree_2, &2).unwrap().key, 2);
        assert!(super::tree_search(tree_2, &3).is_none());
        assert_eq!(super::tree_search(tree_2, &5).unwrap().key, 5);
        assert!(super::tree_search(tree_2, &7).is_none());
        assert_eq!(super::tree_search(tree_2, &8).unwrap().key, 8);
        assert!(super::tree_search(tree_2, &10).is_none());
    }

    #[test]
    fn test_iterative_tree_search() {
        assert!(super::iterative_tree_search(&None, &4).is_none());

        let tree_1 = &make_simple_tree![1];

        assert!(super::iterative_tree_search(tree_1, &0).is_none());
        assert_eq!(super::iterative_tree_search(tree_1, &1).unwrap().key, 1);
        assert!(super::iterative_tree_search(tree_1, &4).is_none());

        let tree_2 = &make_simple_tree![(5, 2, 8)];

        assert!(super::iterative_tree_search(tree_2, &1).is_none());
        assert_eq!(super::iterative_tree_search(tree_2, &2).unwrap().key, 2);
        assert!(super::iterative_tree_search(tree_2, &3).is_none());
        assert_eq!(super::iterative_tree_search(tree_2, &5).unwrap().key, 5);
        assert!(super::iterative_tree_search(tree_2, &7).is_none());
        assert_eq!(super::iterative_tree_search(tree_2, &8).unwrap().key, 8);
        assert!(super::iterative_tree_search(tree_2, &10).is_none());
    }

    #[test]
    fn test_tree_minimum() {
        assert_eq!(super::tree_minimum(&make_simple_tree![1].unwrap()).key, 1);
        assert_eq!(super::tree_minimum(&make_simple_tree![(5, 2, 8)].unwrap()).key, 2);
    }

    #[test]
    fn test_tree_maximum() {
        assert_eq!(super::tree_maximum(&make_simple_tree![1].unwrap()).key, 1);
        assert_eq!(super::tree_maximum(&make_simple_tree![(5, 2, 8)].unwrap()).key, 8);
    }

    #[test]
    fn test_tree_successor() {
        let root = make_tree![(5, 2, (7, 6, 8))].unwrap();
        let left = root.get_left_child().clone().unwrap();
        let right = root.get_right_child().clone().unwrap();
        let right_right = right.get_right_child().clone().unwrap();

        assert_eq!(*super::tree_successor(left).as_ref().unwrap().get_key(), 5);
        assert_eq!(*super::tree_successor(right).as_ref().unwrap().get_key(), 8);
        assert!(super::tree_successor(right_right).is_none());
        assert_eq!(*super::tree_successor(root).as_ref().unwrap().get_key(), 6);
    }
}
