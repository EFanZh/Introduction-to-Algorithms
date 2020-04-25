use super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
use super::section_13_2_rotations::exercises::exercise_13_2_1::right_rotate;
use super::section_13_2_rotations::left_rotate;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn is_left_child<T>(node: &Rc<RefCell<RedBlackTreeNode<T>>>, maybe_parent: &RedBlackTreeNode<T>) -> bool {
    if let Some(left) = maybe_parent.left.as_ref() {
        Rc::ptr_eq(node, left)
    } else {
        false
    }
}

fn is_right_child<T>(node: &Rc<RefCell<RedBlackTreeNode<T>>>, maybe_parent: &RedBlackTreeNode<T>) -> bool {
    if let Some(right) = maybe_parent.right.as_ref() {
        Rc::ptr_eq(node, right)
    } else {
        false
    }
}

fn rotate_2<T, F: FnOnce(&mut Rc<RefCell<RedBlackTreeNode<T>>>)>(
    root: &mut Rc<RefCell<RedBlackTreeNode<T>>>,
    node: &Rc<RefCell<RedBlackTreeNode<T>>>,
    f: F,
) {
    let maybe_parent = node.borrow().p.upgrade(); // Do not inline this.

    if let Some(parent) = maybe_parent {
        let mut parent_ref = parent.borrow_mut();

        if let Some(left) = parent_ref.left.as_mut().filter(|left| Rc::ptr_eq(node, left)) {
            f(left);
        } else {
            f(parent_ref.right.as_mut().unwrap());
        }
    } else {
        f(root);
    }
}

fn left_rotate_2<T>(root: &mut Rc<RefCell<RedBlackTreeNode<T>>>, node: &Rc<RefCell<RedBlackTreeNode<T>>>) {
    rotate_2(root, node, left_rotate);
}

fn right_rotate_2<T>(root: &mut Rc<RefCell<RedBlackTreeNode<T>>>, node: &Rc<RefCell<RedBlackTreeNode<T>>>) {
    rotate_2(root, node, right_rotate);
}

// RB-Insert-Fixup(T, z)
//
//  1  while z.p.color == red
//  2      if z.p == z.p.p.left
//  3          y = z.p.p.right
//  4          if y.color == red
//  5              z.p.color = black       // case 1
//  6              y.color = black         // case 1
//  7              z.p.p.color = red       // case 1
//  8              z = z.p.p               // case 1
//  9          else if z == z.p.right
// 10                   z = z.p            // case 2
// 11                   Left-Rotate(T, z)  // case 2
// 12               z.p.color = black      // case 3
// 13               z.p.p.color = red      // case 3
// 14               Right-Rotate(T, z.p.p) // case 3
// 15      else (same as then clause
//                 with “right” and “left” exchanged)
// 16  T.root.color = black

pub fn rb_insert_fixup<T>(t: &mut Rc<RefCell<RedBlackTreeNode<T>>>, mut z: Rc<RefCell<RedBlackTreeNode<T>>>) {
    loop {
        let mut z_ref = z.borrow_mut();

        if let Some(z_p) = z_ref.p.upgrade() {
            let mut z_p_ref = z_p.borrow_mut();

            if z_p_ref.color == Color::Red {
                let z_p_p = z_p_ref.p.upgrade().unwrap();
                let mut z_p_p_ref = z_p_p.borrow_mut();

                if is_left_child(&z_p, &z_p_p_ref) {
                    if let Some(mut y_ref) = z_p_p_ref
                        .right
                        .clone()
                        .as_ref()
                        .map(|k| k.borrow_mut())
                        .filter(|k| k.color == Color::Red)
                    {
                        // y.color == red.

                        z_p_ref.color = Color::Black;
                        y_ref.color = Color::Black;
                        z_p_p_ref.color = Color::Red;

                        drop((z_ref, z_p_p_ref));

                        z = z_p_p;
                    } else {
                        // y.color == black.

                        z_p_p_ref.color = Color::Red;

                        if is_right_child(&z, &z_p_ref) {
                            z_ref.color = Color::Black;

                            drop((z_ref, z_p_ref));

                            left_rotate(z_p_p_ref.left.as_mut().unwrap());
                        } else {
                            z_p_ref.color = Color::Black;

                            drop(z_p_ref);
                        }

                        // Right rotate z.p.p.

                        drop(z_p_p_ref);

                        right_rotate_2(t, &z_p_p);
                    }
                } else if let Some(mut y_ref) = z_p_p_ref
                    .left
                    .clone()
                    .as_ref()
                    .map(|k| k.borrow_mut())
                    .filter(|k| k.color == Color::Red)
                {
                    // y.color == red.

                    z_p_ref.color = Color::Black;
                    y_ref.color = Color::Black;
                    z_p_p_ref.color = Color::Red;

                    drop((z_ref, z_p_p_ref));

                    z = z_p_p;
                } else {
                    // y.color == black.

                    z_p_p_ref.color = Color::Red;

                    if is_left_child(&z, &z_p_ref) {
                        z_ref.color = Color::Black;

                        drop((z_ref, z_p_ref));

                        right_rotate(z_p_p_ref.right.as_mut().unwrap());
                    } else {
                        z_p_ref.color = Color::Black;

                        drop(z_p_ref);
                    }

                    // Left rotate z.p.p.

                    drop(z_p_p_ref);

                    left_rotate_2(t, &z_p_p);
                }
            } else {
                // z.p is a black node.

                break;
            }
        } else {
            // z is root.

            z_ref.color = Color::Black;

            drop(z_ref);

            *t = z;

            break;
        }
    }
}

// RB-Insert(T, z)
//
//  1  y = T.nil
//  2  x = T.root
//  3  while x ≠ T.nil
//  4      y = x
//  5      if z.key < x.key
//  6          x = x.left
//  7      else x = x.right
//  8  z.p = y
//  9  if y == T.nil
// 10      T.root = z
// 11  elseif z.key < y.key
// 12      y.left = z
// 13  else y.right = z
// 14  z.left = T.nil
// 15  z.right = T.nil
// 16  z.color = red
// 17  RB-Insert-Fixup(T, z)

pub fn rb_insert<T: Ord>(t: &mut Option<Rc<RefCell<RedBlackTreeNode<T>>>>, z: Rc<RefCell<RedBlackTreeNode<T>>>) {
    let mut z_ref = z.borrow_mut();

    if let Some(mut x_rc) = t.clone() {
        loop {
            x_rc = {
                let mut x_ref = x_rc.borrow_mut();

                if z_ref.key < x_ref.key {
                    if let Some(left) = &x_ref.left {
                        left.clone()
                    } else {
                        x_ref.left = Some(z.clone());

                        break;
                    }
                } else if let Some(right) = &x_ref.right {
                    right.clone()
                } else {
                    x_ref.right = Some(z.clone());

                    break;
                }
            };
        }

        z_ref.p = Rc::downgrade(&x_rc);
    } else {
        *t = Some(z.clone());

        z_ref.p = Weak::new();
    }

    z_ref.left = None;
    z_ref.right = None;
    z_ref.color = Color::Red;

    drop(z_ref);

    rb_insert_fixup(t.as_mut().unwrap(), z);
}

#[cfg(test)]
mod tests {
    use super::super::section_13_1_properties_of_red_black_trees::tests::check_valid_red_black_tree;
    use super::super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
    use super::{rb_insert, rb_insert_fixup};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Tree<T> = Option<Rc<RefCell<RedBlackTreeNode<T>>>>;

    fn red<T>(key: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(RedBlackTreeNode::new(Color::Red, key, left, right))
    }

    fn red_leaf<T>(key: T) -> Tree<T> {
        Some(RedBlackTreeNode::new_leaf(Color::Red, key))
    }

    fn black<T>(key: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(RedBlackTreeNode::new(Color::Black, key, left, right))
    }

    fn black_leaf<T>(key: T) -> Tree<T> {
        Some(RedBlackTreeNode::new_leaf(Color::Black, key))
    }

    fn run_rb_insert_fixup_test(mut tree: Tree<i32>, z: Rc<RefCell<RedBlackTreeNode<i32>>>, expected: &Tree<i32>) {
        rb_insert_fixup(tree.as_mut().unwrap(), z);

        check_valid_red_black_tree(&tree);

        assert_eq!(&tree, expected);
    }

    #[test]
    fn test_rb_insert_fixup_root() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);
        let tree = Some(z.clone());

        run_rb_insert_fixup_test(tree, z, &black_leaf(4));
    }

    #[test]
    fn test_rb_insert_fixup_case_1_left_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);

        run_rb_insert_fixup_test(
            black(7, red(5, Some(z.clone()), None), red_leaf(8)),
            z,
            &black(7, black(5, red_leaf(4), None), black_leaf(8)),
        );
    }

    #[test]
    fn test_rb_insert_fixup_case_1_right_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 8);

        run_rb_insert_fixup_test(
            black(5, red_leaf(4), red(7, None, Some(z.clone()))),
            z,
            &black(5, black_leaf(4), black(7, None, red_leaf(8))),
        );
    }

    #[test]
    fn test_rb_insert_fixup_case_2_and_3_left_side() {
        let z = RedBlackTreeNode::new(Color::Red, 7, black_leaf(5), black_leaf(8));

        run_rb_insert_fixup_test(
            black(11, red(2, black_leaf(1), Some(z.clone())), black_leaf(14)),
            z,
            &black(
                7,
                red(2, black_leaf(1), black_leaf(5)),
                red(11, black_leaf(8), black_leaf(14)),
            ),
        )
    }

    #[test]
    fn test_rb_insert_fixup_case_2_and_3_right_side() {
        let z = RedBlackTreeNode::new(Color::Red, 9, black_leaf(8), black_leaf(11));

        run_rb_insert_fixup_test(
            black(5, black_leaf(2), red(14, Some(z.clone()), black_leaf(15))),
            z,
            &black(
                9,
                red(5, black_leaf(2), black_leaf(8)),
                red(14, black_leaf(11), black_leaf(15)),
            ),
        )
    }

    #[test]
    fn test_rb_insert_fixup_case_3_left_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 0);

        run_rb_insert_fixup_test(
            black(3, black(2, red(1, Some(z.clone()), None), None), black_leaf(4)),
            z,
            &black(3, black(1, red_leaf(0), red_leaf(2)), black_leaf(4)),
        )
    }

    #[test]
    fn test_rb_insert_fixup_case_3_right_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);

        run_rb_insert_fixup_test(
            black(1, black_leaf(0), black(2, None, red(3, None, Some(z.clone())))),
            z,
            &black(1, black_leaf(0), black(3, red_leaf(2), red_leaf(4))),
        )
    }

    #[test]
    fn test_rb_insert_fixup_full_left_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);

        run_rb_insert_fixup_test(
            black(
                11,
                red(2, black_leaf(1), black(7, red(5, Some(z.clone()), None), red_leaf(8))),
                black(14, None, red_leaf(15)),
            ),
            z,
            &black(
                7,
                red(2, black_leaf(1), black(5, red_leaf(4), None)),
                red(11, black_leaf(8), black(14, None, red_leaf(15))),
            ),
        )
    }

    #[test]
    fn test_rb_insert_fixup_full_right_side() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 12);

        run_rb_insert_fixup_test(
            black(
                5,
                black(2, red_leaf(1), None),
                red(
                    14,
                    black(9, red_leaf(8), red(11, None, Some(z.clone()))),
                    black_leaf(15),
                ),
            ),
            z,
            &black(
                9,
                red(5, black(2, red_leaf(1), None), black_leaf(8)),
                red(14, black(11, None, red_leaf(12)), black_leaf(15)),
            ),
        )
    }

    #[test]
    fn test_rb_insert() {
        let mut tree = None;

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 4));

        assert_eq!(&tree, &black_leaf(4));

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 2));

        assert_eq!(&tree, &black(4, red_leaf(2), None));

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 6));

        assert_eq!(&tree, &black(4, red_leaf(2), red_leaf(6)));

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 1));

        assert_eq!(&tree, &black(4, black(2, red_leaf(1), None), black_leaf(6)));

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 3));

        assert_eq!(&tree, &black(4, black(2, red_leaf(1), red_leaf(3)), black_leaf(6)));

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 5));

        assert_eq!(
            &tree,
            &black(4, black(2, red_leaf(1), red_leaf(3)), black(6, red_leaf(5), None))
        );

        rb_insert(&mut tree, RedBlackTreeNode::new_leaf(Color::Red, 7));

        assert_eq!(
            &tree,
            &black(
                4,
                black(2, red_leaf(1), red_leaf(3)),
                black(6, red_leaf(5), red_leaf(7))
            )
        );
    }
}
