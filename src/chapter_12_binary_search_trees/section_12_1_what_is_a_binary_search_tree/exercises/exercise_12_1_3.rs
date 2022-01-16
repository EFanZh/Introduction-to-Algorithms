use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;

pub fn iterative_inorder_tree_walk<T, F: FnMut(&T)>(mut root: &Option<Box<SimpleBinaryTreeNode<T>>>, mut f: F) {
    let mut stack = Vec::new();

    loop {
        if let Some(node) = root {
            root = &node.left;
            stack.push(node);
        } else if let Some(node) = stack.pop() {
            f(&node.key);

            root = &node.right;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::iterative_inorder_tree_walk;
    use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;
    use crate::make_simple_tree;

    fn inorder_tree_walk_as_vec(root: &Option<Box<SimpleBinaryTreeNode<i32>>>) -> Vec<i32> {
        let mut result = Vec::new();

        iterative_inorder_tree_walk(root, |&key| result.push(key));

        result
    }

    #[test]
    fn test_iterative_inorder_tree_walk() {
        assert_eq!(inorder_tree_walk_as_vec(&None), vec![]);
        assert_eq!(inorder_tree_walk_as_vec(&make_simple_tree![1]), vec![1]);
        assert_eq!(inorder_tree_walk_as_vec(&make_simple_tree![(1, 2, 3)]), vec![2, 1, 3]);

        assert_eq!(
            inorder_tree_walk_as_vec(&make_simple_tree![(1, (2, 3, 4), 5)]),
            vec![3, 2, 4, 1, 5]
        );
    }
}
