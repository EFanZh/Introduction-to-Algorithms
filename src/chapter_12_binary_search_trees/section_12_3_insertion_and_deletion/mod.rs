use super::super::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;
use std::mem;

pub mod exercises;

// Tree-Insert(T, z)
//
//  1  y = nil
//  2  x = T.root
//  3  while x ≠ nil
//  4      y = x
//  5      if z.key < x.key
//  6          x = x.left
//  7      else x = x.right
//  8  z.p = y
//  9  if y == nil
// 10      T.root = z // tree T was empty
// 11  elseif z.key < y.key
// 12      y.left = z
// 13  else y.right = z

pub fn tree_insert<T: Ord>(mut x: &mut Option<Box<SimpleBinaryTreeNode<T>>>, z: Box<SimpleBinaryTreeNode<T>>) {
    // Double pointer rules!

    while let Some(x_2) = x {
        if z.key < x_2.key {
            x = &mut x_2.left;
        } else {
            x = &mut x_2.right;
        }
    }

    *x = Some(z);
}

pub fn lift_min<T>(mut root: Box<SimpleBinaryTreeNode<T>>) -> Box<SimpleBinaryTreeNode<T>> {
    if root.left.is_none() {
        root
    } else {
        let mut node_ref = &mut root.left;

        let min_right = loop {
            let node = node_ref.as_mut().unwrap();

            if node.left.is_some() {
                node_ref = &mut node_ref.as_mut().unwrap().left; // See https://github.com/rust-lang/rust/issues/63908.
            } else {
                break node.right.take();
            }
        };

        let mut min = mem::replace(node_ref, min_right).unwrap();

        min.right = Some(root);

        min
    }
}

// Tree-Delete(T, z)
//
//  1  if z.left == nil
//  2      Transplant(T, z, z.right)
//  3  elseif z.right == nil
//  4      Transplant(T, z, z.left)
//  5  else y = Tree-Minimum(z.right)
//  6      if y.p ≠ z
//  7          Transplant(T, y, y.right)
//  8          y.right = z.right
//  9          y.right.p = y
// 10      Transplant(T, z, y)
// 11      y.left = z.left
// 12      y.left.p = y

pub fn tree_delete<T>(z: &mut Option<Box<SimpleBinaryTreeNode<T>>>) {
    let z_unwrapped = z.as_mut().unwrap();

    if z_unwrapped.left.is_none() {
        // The node being deleted doesn’t have left child.

        *z = z_unwrapped.right.take();
    } else if let Some(z_right) = z_unwrapped.right.take() {
        let mut new_root = lift_min(z_right);

        new_root.left = z_unwrapped.left.take();

        *z = Some(new_root);
    } else {
        // The node being deleted doesn’t have right child.

        *z = z_unwrapped.left.take();
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;
    use super::{tree_delete, tree_insert};
    use crate::make_simple_tree;

    pub fn run_tree_insert_tests<
        F: FnMut(&mut Option<Box<SimpleBinaryTreeNode<i32>>>, Box<SimpleBinaryTreeNode<i32>>),
    >(
        mut f: F,
    ) {
        let mut insert = move |mut node: Option<Box<SimpleBinaryTreeNode<i32>>>,
                               value: i32|
              -> Option<Box<SimpleBinaryTreeNode<i32>>> {
            f(&mut node, SimpleBinaryTreeNode::new_leaf(value));

            node
        };

        assert_eq!(insert(make_simple_tree![()], 4), make_simple_tree![4]);
        assert_eq!(insert(make_simple_tree![3], 1), make_simple_tree![(3, 1, ())]);
        assert_eq!(insert(make_simple_tree![3], 4), make_simple_tree![(3, (), 4)]);

        assert_eq!(
            insert(make_simple_tree![(5, 3, ())], 1),
            make_simple_tree![(5, (3, 1, ()), ())]
        );

        assert_eq!(
            insert(make_simple_tree![(5, 3, ())], 4),
            make_simple_tree![(5, (3, (), 4), ())]
        );

        assert_eq!(insert(make_simple_tree![(5, 3, ())], 6), make_simple_tree![(5, 3, 6)]);
    }

    #[test]
    fn test_tree_insert() {
        run_tree_insert_tests(tree_insert);
    }

    #[test]
    fn test_tree_delete() {
        fn delete(mut node: Option<Box<SimpleBinaryTreeNode<i32>>>) -> Option<Box<SimpleBinaryTreeNode<i32>>> {
            tree_delete(&mut node);

            node
        }

        // Only one node.

        assert_eq!(delete(make_simple_tree![1]), make_simple_tree![()]);

        // Right child is empty.

        assert_eq!(delete(make_simple_tree![(2, 1, ())]), make_simple_tree![1]);

        // Left child is empty.

        assert_eq!(delete(make_simple_tree![(2, (), 3)]), make_simple_tree![3]);

        // The minimum element in the right subtree is the right child.

        assert_eq!(
            delete(make_simple_tree![(2, 1, (3, (), 4))]),
            make_simple_tree![(3, 1, 4)]
        );

        // The minimum element in the right subtree is not the right child.

        assert_eq!(
            delete(make_simple_tree![(2, 1, (5, (3, (), 4), ()))]),
            make_simple_tree![(3, 1, (5, 4, ()))]
        );
    }
}
