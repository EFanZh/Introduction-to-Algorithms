use super::super::BinaryTreeNode;

pub fn iterate_tree<T, F: FnMut(&T)>(root: &Option<Box<BinaryTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Box<BinaryTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            f(&node.key);

            helper(&node.left, f);
            helper(&node.right, f);
        }
    }

    helper(root, &mut f);
}

#[cfg(test)]
mod tests {
    use super::super::super::BinaryTreeNode;
    use super::iterate_tree;
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::iter;

    type MaybeNode<T> = Option<Box<BinaryTreeNode<T>>>;

    fn collect_tree_elements<T: Clone + Eq + Hash>(root: &MaybeNode<T>) -> HashSet<T> {
        let mut result = HashSet::new();

        iterate_tree(root, |key| {
            result.insert(key.clone());
        });

        result
    }

    fn run_single_tests<I: IntoIterator<Item = i32>>(root: MaybeNode<i32>, result: I) {
        assert_eq!(collect_tree_elements(&root), result.into_iter().collect());
    }

    fn make_node<T>(key: T, left: MaybeNode<T>, right: MaybeNode<T>) -> MaybeNode<T> {
        Some(BinaryTreeNode::new_boxed(key, left, right))
    }

    fn make_leaf_node<T>(key: T) -> MaybeNode<T> {
        Some(BinaryTreeNode::boxed_leaf(key))
    }

    #[test]
    fn test_iterate_tree() {
        run_single_tests(None, iter::empty());
        run_single_tests(make_leaf_node(4), iter::once(4));
        run_single_tests(make_node(4, make_leaf_node(5), None), vec![4, 5]);
        run_single_tests(make_node(4, make_leaf_node(5), make_leaf_node(6)), vec![4, 5, 6]);

        run_single_tests(
            make_node(4, make_leaf_node(5), make_node(6, make_leaf_node(7), None)),
            vec![4, 5, 6, 7],
        );
    }
}
