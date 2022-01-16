use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;

pub mod exercises;

// Inorder-Tree-Walk(x)
//
// 1  if x ≠ nil
// 2      Inorder-Tree-Walk(x.left)
// 3      print x.key
// 4      Inorder-Tree-Walk(x.right)

pub fn inorder_tree_walk<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            helper(&node.left, f);

            f(&node.key);

            helper(&node.right, f);
        }
    }

    helper(root, &mut f);
}

#[cfg(test)]
mod tests {
    use super::inorder_tree_walk;
    use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;
    use crate::make_simple_tree;

    fn inorder_tree_walk_as_vec(root: &Option<Box<SimpleBinaryTreeNode<i32>>>) -> Vec<i32> {
        let mut result = Vec::new();

        inorder_tree_walk(root, |&key| result.push(key));

        result
    }

    #[test]
    fn test_inorder_tree_walk() {
        assert_eq!(inorder_tree_walk_as_vec(&None), vec![]);
        assert_eq!(inorder_tree_walk_as_vec(&make_simple_tree![1]), vec![1]);
        assert_eq!(inorder_tree_walk_as_vec(&make_simple_tree![(1, 2, 3)]), vec![2, 1, 3]);

        assert_eq!(
            inorder_tree_walk_as_vec(&make_simple_tree![(1, (2, 3, 4), 5)]),
            vec![3, 2, 4, 1, 5]
        );
    }
}
