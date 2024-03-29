use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;

pub fn recursive_preorder_tree_walk<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            f(&node.key);

            helper(&node.left, f);
            helper(&node.right, f);
        }
    }

    helper(root, &mut f);
}

pub fn recursive_postorder_tree_walk<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Box<SimpleBinaryTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            helper(&node.left, f);
            helper(&node.right, f);

            f(&node.key);
        }
    }

    helper(root, &mut f);
}

#[cfg(test)]
mod tests {
    use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;
    use crate::make_simple_tree;

    fn preorder_tree_walk_as_vec(root: &Option<Box<SimpleBinaryTreeNode<i32>>>) -> Vec<i32> {
        let mut result = Vec::new();

        super::recursive_preorder_tree_walk(root, |&key| result.push(key));

        result
    }

    fn postorder_tree_walk_as_vec(root: &Option<Box<SimpleBinaryTreeNode<i32>>>) -> Vec<i32> {
        let mut result = Vec::new();

        super::recursive_postorder_tree_walk(root, |&key| result.push(key));

        result
    }

    #[test]
    fn test_recursive_preorder_tree_walk() {
        assert_eq!(preorder_tree_walk_as_vec(&None), vec![]);
        assert_eq!(preorder_tree_walk_as_vec(&make_simple_tree![1]), vec![1]);
        assert_eq!(preorder_tree_walk_as_vec(&make_simple_tree![(1, 2, 3)]), vec![1, 2, 3]);

        assert_eq!(
            preorder_tree_walk_as_vec(&make_simple_tree![(1, (2, 3, 4), 5)]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_recursive_postorder_tree_walk() {
        assert_eq!(postorder_tree_walk_as_vec(&None), vec![]);
        assert_eq!(postorder_tree_walk_as_vec(&make_simple_tree![1]), vec![1]);
        assert_eq!(postorder_tree_walk_as_vec(&make_simple_tree![(1, 2, 3)]), vec![2, 3, 1]);

        assert_eq!(
            postorder_tree_walk_as_vec(&make_simple_tree![(1, (2, 3, 4), (5))]),
            vec![3, 4, 2, 5, 1]
        );
    }
}
