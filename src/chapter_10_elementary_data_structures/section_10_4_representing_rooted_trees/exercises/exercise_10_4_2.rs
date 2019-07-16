use super::super::BinaryTreeNode;
use std::rc::Rc;

pub fn iterate_tree<T, F: FnMut(&T)>(root: &Option<Rc<BinaryTreeNode<T>>>, mut f: F) {
    fn helper<T, F: FnMut(&T)>(root: &Option<Rc<BinaryTreeNode<T>>>, f: &mut F) {
        if let Some(node) = root {
            f(node.get_key());

            helper(node.get_left_child(), f);
            helper(node.get_right_child(), f);
        }
    }

    helper(root, &mut f);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::super::super::BinaryTreeNode;
    use super::iterate_tree;
    use std::collections::HashSet;
    use std::iter;
    use std::rc::Rc;

    type MaybeNode<T> = Option<Rc<BinaryTreeNode<T>>>;

    fn run_single_tests<F: FnOnce(&MaybeNode<i32>) -> HashSet<i32>, I: IntoIterator<Item = i32>>(
        f: F,
        root: MaybeNode<i32>,
        result: I,
    ) {
        assert_eq!(f(&root), result.into_iter().collect());
    }

    fn make_node<T>(key: T, left: MaybeNode<T>, right: MaybeNode<T>) -> MaybeNode<T> {
        Some(BinaryTreeNode::new_boxed(key, left, right))
    }

    fn make_leaf_node<T>(key: T) -> MaybeNode<T> {
        Some(BinaryTreeNode::boxed_leaf(key))
    }

    pub fn run_iterate_tree_tests<F: FnMut(&MaybeNode<i32>) -> HashSet<i32>>(mut f: F) {
        run_single_tests(&mut f, None, iter::empty());
        run_single_tests(&mut f, make_leaf_node(4), iter::once(4));
        run_single_tests(&mut f, make_node(4, make_leaf_node(5), None), vec![4, 5]);

        run_single_tests(
            &mut f,
            make_node(4, make_leaf_node(5), make_leaf_node(6)),
            vec![4, 5, 6],
        );

        run_single_tests(
            &mut f,
            make_node(4, make_leaf_node(5), make_node(6, make_leaf_node(7), None)),
            vec![4, 5, 6, 7],
        );
    }

    #[test]
    fn test_iterate_tree() {
        run_iterate_tree_tests(|root| {
            let mut result = HashSet::new();

            iterate_tree(root, |key| {
                result.insert(key.clone());
            });

            result
        });
    }
}