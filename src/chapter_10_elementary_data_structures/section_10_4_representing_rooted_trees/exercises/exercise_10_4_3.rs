use super::super::BinaryTreeNode;
use std::rc::Rc;

pub fn iterate_tree_1<T, F: FnMut(&T)>(root: &Option<Rc<BinaryTreeNode<T>>>, mut f: F) {
    let mut stack = Vec::new();
    let mut top = root;

    loop {
        if let Some(node) = top {
            f(node.get_key());

            stack.push(node.get_left_child());

            top = node.get_right_child();
        } else if let Some(new_top) = stack.pop() {
            top = new_top;
        } else {
            break;
        }
    }
}

pub fn iterate_tree_2<T, F: FnMut(&T)>(root: &Option<Rc<BinaryTreeNode<T>>>, mut f: F) {
    if let Some(root_node) = root {
        let mut stack = vec![root_node.as_ref()];

        while let Some(top) = stack.pop() {
            f(top.get_key());

            if let Some(left_node) = top.get_left_child() {
                stack.push(left_node.as_ref());
            }

            if let Some(right_node) = top.get_right_child() {
                stack.push(right_node.as_ref());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::exercise_10_4_2::tests::run_iterate_tree_test_cases;
    use super::{iterate_tree_1, iterate_tree_2};
    use std::collections::HashSet;

    #[test]
    fn test_iterate_tree_1() {
        run_iterate_tree_test_cases(|root| {
            let mut result = HashSet::new();

            iterate_tree_1(root, |key| {
                result.insert(*key);
            });

            result
        });
    }

    #[test]
    fn test_iterate_tree_2() {
        run_iterate_tree_test_cases(|root| {
            let mut result = HashSet::new();

            iterate_tree_2(root, |key| {
                result.insert(*key);
            });

            result
        });
    }
}
