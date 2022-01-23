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
    use crate::make_unbounded_branching_tree;
    use std::collections::HashSet;
    use std::iter;
    use std::rc::Rc;

    type MaybeNode<T> = Option<Rc<UnboundedBranchingTreeNode<T>>>;

    fn run_single_tests<I: IntoIterator<Item = i32>>(root: &MaybeNode<i32>, result: I) {
        let mut collected_elements = HashSet::new();

        super::iterate_tree(root, |&key| {
            collected_elements.insert(key);
        });

        assert_eq!(collected_elements, result.into_iter().collect());
    }

    #[test]
    fn test_iterate_tree() {
        run_single_tests(&None, iter::empty());
        run_single_tests(&make_unbounded_branching_tree![4], iter::once(4));
        run_single_tests(&make_unbounded_branching_tree![(4, 5, ())], vec![4, 5]);
        run_single_tests(&make_unbounded_branching_tree![(4, (5, (), 6), ())], vec![4, 5, 6]);

        run_single_tests(
            &make_unbounded_branching_tree![(4, (5, (), (6, 7, ())), ())],
            vec![4, 5, 6, 7],
        );
    }
}
