use super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
use super::section_13_2_rotations::exercises::exercise_13_2_1::right_rotate;
use super::section_13_2_rotations::left_rotate;
use std::cell::RefCell;
use std::rc::Rc;

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

pub fn rb_insert_fixup<T>(t: &mut Option<Rc<RefCell<RedBlackTreeNode<T>>>>, mut z: Rc<RefCell<RedBlackTreeNode<T>>>) {
    loop {
        let mut z_ref = z.borrow_mut();

        if let Some(z_p) = z_ref.p.upgrade() {
            let mut z_p_ref = z_p.borrow_mut();

            if z_p_ref.color == Color::Red {
                let z_p_p = z_p_ref.p.upgrade().unwrap();
                let mut z_p_p_ref = z_p_p.borrow_mut();

                if is_left_child(&z_p, &z_p_p_ref) {
                    if let Some(y) = z_p_p_ref.right.clone() {
                        let mut y_ref = y.borrow_mut();

                        if y_ref.color == Color::Red {
                            // y.color == red.

                            z_p_ref.color = Color::Black;
                            y_ref.color = Color::Black;
                            z_p_p_ref.color = Color::Red;

                            drop(z_ref);
                            drop(z_p_p_ref);

                            z = z_p_p;

                            continue;
                        }
                    }

                    // y.color == black.

                    z_p_p_ref.color = Color::Red;

                    if is_right_child(&z, &z_p_ref) {
                        z_ref.color = Color::Black;

                        drop(z_ref);
                        drop(z_p_ref);

                        left_rotate(&mut z_p_p_ref.left);
                    } else {
                        z_p_ref.color = Color::Black;
                    }

                    // Right rotate z.p.p;

                    if let Some(z_p_p_p) = z_p_p_ref.p.upgrade() {
                        let mut z_p_p_p_ref = z_p_p_p.borrow_mut();

                        if is_left_child(&z_p_p, &z_p_p_p_ref) {
                            right_rotate(&mut z_p_p_p_ref.left);
                        } else {
                            right_rotate(&mut z_p_p_p_ref.right);
                        }
                    } else {
                        drop(z_p_p_ref);

                        right_rotate(t);
                    }
                } else {
                    unimplemented!();
                }
            } else {
                // z.p is a black node.

                break;
            }
        } else {
            // z is root.

            z_ref.color = Color::Black;

            drop(z_ref);

            *t = Some(z);

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

#[cfg(test)]
mod tests {
    use super::super::section_13_1_properties_of_red_black_trees::tests::check_valid_red_black_tree;
    use super::super::section_13_1_properties_of_red_black_trees::{Color, RedBlackTreeNode};
    use super::rb_insert_fixup;
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

    #[test]
    fn test_rb_insert_fixup_root() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);
        let mut tree = Some(z.clone());

        rb_insert_fixup(&mut tree, z);

        check_valid_red_black_tree(&tree);

        assert_eq!(tree, black_leaf(4));
    }

    #[test]
    fn test_rb_insert_fixup_case_1() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);
        let mut tree = black(7, red(5, Some(z.clone()), None), red_leaf(8));

        rb_insert_fixup(&mut tree, z);

        check_valid_red_black_tree(&tree);

        assert_eq!(tree, black(7, black(5, red_leaf(4), None), black_leaf(8)));
    }

    #[test]
    fn test_rb_insert_fixup_case_2_and_3() {
        let z = RedBlackTreeNode::new(Color::Red, 7, black_leaf(5), black_leaf(8));
        let mut tree = black(11, red(2, black_leaf(1), Some(z.clone())), black_leaf(14));

        rb_insert_fixup(&mut tree, z);

        check_valid_red_black_tree(&tree);

        assert_eq!(
            tree,
            black(
                7,
                red(2, black_leaf(1), black_leaf(5)),
                red(11, black_leaf(8), black_leaf(14))
            )
        );
    }

    #[test]
    fn test_rb_insert_fixup_full() {
        let z = RedBlackTreeNode::new_leaf(Color::Red, 4);
        let mut tree = black(
            11,
            red(2, black_leaf(1), black(7, red(5, Some(z.clone()), None), red_leaf(8))),
            black(14, None, red_leaf(15)),
        );

        rb_insert_fixup(&mut tree, z);

        check_valid_red_black_tree(&tree);

        assert_eq!(
            tree,
            black(
                7,
                red(2, black_leaf(1), black(5, red_leaf(4), None)),
                red(11, black_leaf(8), black(14, None, red_leaf(15)))
            )
        );
    }
}
