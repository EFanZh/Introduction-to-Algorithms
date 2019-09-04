use super::super::super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;
use std::cell::RefCell;
use std::mem;
use std::rc::{Rc, Weak};

pub fn right_rotate<T>(y: &mut Option<Rc<RefCell<RedBlackTreeNode<T>>>>) {
    // Break y from parent.

    let y_rc = y.take().unwrap();
    let mut y_ref = y_rc.borrow_mut();
    let p_weak = mem::replace(&mut y_ref.p, Weak::new());

    // Break x from y.

    let x_rc = y_ref.left.take().unwrap();
    let mut x_ref = x_rc.borrow_mut();
    let y_weak = mem::replace(&mut x_ref.p, Weak::new());

    // Attach x.right to y.left.

    if let Some(x_right) = x_ref.right.take() {
        x_right.borrow_mut().p = y_weak;
        y_ref.left = Some(x_right);
    }

    // Attach y to x.right;

    y_ref.p = Rc::downgrade(&x_rc);

    drop(y_ref);

    x_ref.right = Some(y_rc);

    // Attach x to parent.

    x_ref.p = p_weak;

    drop(x_ref);

    *y = Some(x_rc);
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_13_1_properties_of_red_black_trees::tests::check_valid_tree;
    use super::super::super::super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
    use super::right_rotate;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Tree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

    fn make_node<T>(key: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(RedBlackTreeNode::new(Color::Black, key, left, right))
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = make_node(
            4,
            make_node(2, make_node(1, None, None), make_node(3, None, None)),
            make_node(5, None, None),
        );

        right_rotate(&mut tree);

        check_valid_tree(&tree);

        assert_eq!(
            tree,
            make_node(
                2,
                make_node(1, None, None),
                make_node(4, make_node(3, None, None), make_node(5, None, None)),
            )
        );
    }
}
