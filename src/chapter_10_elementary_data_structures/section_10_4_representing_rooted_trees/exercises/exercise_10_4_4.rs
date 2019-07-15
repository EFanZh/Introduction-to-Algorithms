use super::super::UnboundedBranchingTreeNode;

pub fn iterate_tree<T, F: FnMut(&T)>(root: &Option<Box<UnboundedBranchingTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Box<UnboundedBranchingTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            f(&node.key);

            helper(&node.left_child, f);
            helper(&node.right_sibling, f);
        }
    }

    helper(root, &mut f);
}

#[cfg(test)]
mod tests {

    use super::super::super::UnboundedBranchingTreeNode;
    use super::iterate_tree;
    use std::collections::HashSet;
    use std::iter;

    type MaybeNode<T> = Option<Box<UnboundedBranchingTreeNode<T>>>;

    fn run_single_tests<I: IntoIterator<Item = i32>>(root: MaybeNode<i32>, result: I) {
        let mut collected_elements = HashSet::new();

        iterate_tree(&root, |&key| {
            collected_elements.insert(key);
        });

        assert_eq!(collected_elements, result.into_iter().collect());
    }

    fn make_node<T>(key: T, left_child: MaybeNode<T>, right_sibling: MaybeNode<T>) -> MaybeNode<T> {
        Some(UnboundedBranchingTreeNode::new_boxed(key, left_child, right_sibling))
    }

    fn make_leaf_node<T>(key: T) -> MaybeNode<T> {
        Some(UnboundedBranchingTreeNode::boxed_leaf(key))
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
