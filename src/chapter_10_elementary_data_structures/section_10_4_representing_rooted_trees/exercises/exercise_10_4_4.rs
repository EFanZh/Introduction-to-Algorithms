use super::super::UnboundedBranchingTreeNode;
use std::rc::Rc;

pub fn iterate_tree<T, F: FnMut(&T)>(root: &Option<Rc<UnboundedBranchingTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Rc<UnboundedBranchingTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            f(node.get_key());

            helper(node.get_left_child(), f);
            helper(node.get_right_sibling(), f);
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
    use std::rc::Rc;

    type MaybeNode<T> = Option<Rc<UnboundedBranchingTreeNode<T>>>;

    fn run_single_tests<I: IntoIterator<Item = i32>>(root: MaybeNode<i32>, result: I) {
        let mut collected_elements = HashSet::new();

        iterate_tree(&root, |&key| {
            collected_elements.insert(key);
        });

        assert_eq!(collected_elements, result.into_iter().collect());
    }

    fn make_node<T>(key: T, left_child: MaybeNode<T>, right_sibling: MaybeNode<T>) -> MaybeNode<T> {
        Some(UnboundedBranchingTreeNode::new(key, left_child, right_sibling))
    }

    fn make_leaf_node<T>(key: T) -> MaybeNode<T> {
        Some(UnboundedBranchingTreeNode::new_leaf(key))
    }

    #[test]
    fn test_iterate_tree() {
        run_single_tests(None, iter::empty());
        run_single_tests(make_leaf_node(4), iter::once(4));
        run_single_tests(make_node(4, make_leaf_node(5), None), vec![4, 5]);
        run_single_tests(make_node(4, make_node(5, None, make_leaf_node(6)), None), vec![4, 5, 6]);

        run_single_tests(
            make_node(4, make_node(5, None, make_node(6, make_leaf_node(7), None)), None),
            vec![4, 5, 6, 7],
        );
    }
}
