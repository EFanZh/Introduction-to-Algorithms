use super::super::UnboundedBranchingTreeNode;
use std::rc::Rc;

pub fn iterate_tree<T, F: FnMut(&T)>(root: &Option<Rc<UnboundedBranchingTreeNode<T>>>, mut f: F) {
    if let Some(root_ref) = root {
        let mut current_node = root_ref.clone();

        loop {
            f(current_node.get_key());

            if let Some(left_child) = current_node.get_left_child() {
                // Traverse first child.

                current_node = left_child.clone();
            } else if let Some(right_sibling) = current_node.get_right_sibling() {
                // Traverse right sibling.

                current_node = right_sibling.clone();
            } else {
                // Traverse parent’s right sibling.

                loop {
                    if let Some(parent) = current_node.get_parent() {
                        if let Some(parent_right_sibling) = parent.get_right_sibling() {
                            current_node = parent_right_sibling.clone();

                            break;
                        } else {
                            // No parent’s right sibling, go to one level up.

                            current_node = parent.clone();
                        }
                    } else {
                        return;
                    }
                }
            }
        }
    }
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
