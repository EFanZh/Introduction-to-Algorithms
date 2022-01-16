use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::BinaryTreeNode;
use std::rc::Rc;

pub fn tree_predecessor<T: Ord>(mut x: Rc<BinaryTreeNode<T>>) -> Option<Rc<BinaryTreeNode<T>>> {
    fn tree_maximum<T: Ord>(mut x: &Rc<BinaryTreeNode<T>>) -> &Rc<BinaryTreeNode<T>> {
        while let Some(right) = x.get_right_child() {
            x = right;
        }

        x
    }

    if let Some(left) = x.get_left_child() {
        Some(Rc::clone(tree_maximum(left)))
    } else {
        let mut y = x.get_parent();

        while let Some(y_rc) = &y {
            if let Some(y_left_child) = y_rc.get_left_child() {
                if Rc::ptr_eq(&x, y_left_child) {
                    x = Rc::clone(y_rc);
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
    use super::tree_predecessor;
    use crate::make_tree;

    #[test]
    fn test_tree_successor() {
        let root = make_tree![(5, (3, 2, 4), 8)].unwrap();
        let left = root.get_left_child().clone().unwrap();
        let left_left = left.get_left_child().clone().unwrap();
        let right = root.get_right_child().clone().unwrap();

        assert_eq!(*tree_predecessor(left).as_ref().unwrap().get_key(), 2);
        assert!(tree_predecessor(left_left).is_none());
        assert_eq!(*tree_predecessor(right).as_ref().unwrap().get_key(), 5);
        assert_eq!(*tree_predecessor(root).as_ref().unwrap().get_key(), 4);
    }
}
