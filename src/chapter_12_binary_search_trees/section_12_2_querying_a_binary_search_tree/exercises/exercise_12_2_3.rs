use super::super::super::super::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::BinaryTreeNode;
use std::rc::Rc;

pub fn tree_predecessor<T: Ord>(mut x: Rc<BinaryTreeNode<T>>) -> Option<Rc<BinaryTreeNode<T>>> {
    fn tree_maximum<T: Ord>(mut x: &Rc<BinaryTreeNode<T>>) -> &Rc<BinaryTreeNode<T>> {
        while let Some(right) = x.get_right_child() {
            x = right;
        }

        x
    }

    if let Some(left) = x.get_left_child() {
        Some(tree_maximum(left).clone())
    } else {
        let mut y = x.get_parent();

        while let Some(y_rc) = &y {
            if let Some(y_left_child) = y_rc.get_left_child() {
                if Rc::ptr_eq(&x, y_left_child) {
                    x = y_rc.clone();
                    y = y_rc.get_parent();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        y
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::BinaryTreeNode;
    use super::tree_predecessor;
    use crate::make_tree;

    #[test]
    fn test_tree_successor() {
        let root = make_tree![(5, 2, 8)].unwrap();
        let left = root.get_left_child().clone().unwrap();
        let right = root.get_right_child().clone().unwrap();

        assert_eq!(*tree_predecessor(root.clone()).as_ref().unwrap().get_key(), 2);
        assert!(tree_predecessor(left).is_none());
        assert_eq!(*tree_predecessor(right).as_ref().unwrap().get_key(), 5);
    }
}
