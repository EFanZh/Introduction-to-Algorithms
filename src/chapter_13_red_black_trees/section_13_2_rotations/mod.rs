use super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub mod exercises;

// Left-Rotate(T, x)
//
//  1  y = x.right           // set y
//  2  x.right = y.left      // turn y’s left subtree into x’s right subtree
//  3  if y.left ≠ T.nil
//  4  y.left.p = x
//  5      y.p = x.p         // link x’s parent to y
//  6  if x.p == T.nil
//  7      T.root = y
//  8  elseif x == x.p.left
//  9      x.p.left = y
// 10  else x.p.right = y
// 11  y.left = x            // put x on y’s left
// 12  x.p = y

pub fn left_rotate<T>(x_rc: &mut Rc<RefCell<RedBlackTreeNode<T>>>) {
    let mut x_ref = x_rc.borrow_mut();

    // Break y from x.

    let y_rc = x_ref.right.take().unwrap();
    let mut y_ref = y_rc.borrow_mut();
    let x_weak = mem::replace(&mut y_ref.p, mem::take(&mut x_ref.p));

    // Attach y.left to x.right.

    if let Some(y_left) = y_ref.left.take() {
        x_ref.p = mem::replace(&mut y_left.borrow_mut().p, x_weak);
        x_ref.right = Some(y_left);
    } else {
        x_ref.p = Rc::downgrade(&y_rc);
    };

    // Change root to y and attach x to y.left.

    drop((x_ref, y_ref));

    x_rc.borrow_mut().left = Some(mem::replace(x_rc, y_rc));
}

#[cfg(test)]
mod tests {
    use super::super::section_13_1_properties_of_red_black_trees::tests::check_valid_tree;
    use super::super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
    use super::left_rotate;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Tree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

    #[allow(clippy::unnecessary_wraps)]
    fn make_node<T>(key: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(RedBlackTreeNode::new(Color::Black, key, left, right))
    }

    #[test]
    fn test_left_rotate() {
        let mut tree = make_node(
            2,
            make_node(1, None, None),
            make_node(4, make_node(3, None, None), make_node(5, None, None)),
        );

        left_rotate(tree.as_mut().unwrap());

        check_valid_tree(&tree);

        assert_eq!(
            tree,
            make_node(
                4,
                make_node(2, make_node(1, None, None), make_node(3, None, None)),
                make_node(5, None, None)
            )
        );
    }
}
