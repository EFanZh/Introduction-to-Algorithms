use super::super::super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub fn right_rotate<T>(x_rc: &mut Rc<RefCell<RedBlackTreeNode<T>>>) {
    let mut x_ref = x_rc.borrow_mut();

    // Break y from x.

    let y_rc = x_ref.left.take().unwrap();
    let mut y_ref = y_rc.borrow_mut();
    let x_weak = mem::replace(&mut y_ref.p, mem::take(&mut x_ref.p));

    // Attach y.right to x.left.

    if let Some(y_right) = y_ref.right.take() {
        x_ref.p = mem::replace(&mut y_right.borrow_mut().p, x_weak);
        x_ref.left = Some(y_right);
    } else {
        x_ref.p = Rc::downgrade(&y_rc);
    };

    // Change root to y and attach x to y.right.

    drop((x_ref, y_ref));

    x_rc.borrow_mut().right = Some(mem::replace(x_rc, y_rc));
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_13_1_properties_of_red_black_trees::{self, Color, RedBlackTreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Tree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

    fn make_node<T>(key: T, left: impl Into<Tree<T>>, right: impl Into<Tree<T>>) -> Rc<RefCell<RedBlackTreeNode<T>>> {
        RedBlackTreeNode::new(Color::Black, key, left.into(), right.into())
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = make_node(
            4,
            make_node(2, make_node(1, None, None), make_node(3, None, None)),
            make_node(5, None, None),
        );

        super::right_rotate(&mut tree);

        assert_eq!(
            tree,
            make_node(
                2,
                make_node(1, None, None),
                make_node(4, make_node(3, None, None), make_node(5, None, None)),
            )
        );

        section_13_1_properties_of_red_black_trees::tests::check_valid_tree(&Some(tree));
    }
}
