use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct RedBlackTreeNode<T> {
    pub color: Color,
    pub key: T,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
    pub p: Weak<RefCell<Self>>,
}

impl<T> RedBlackTreeNode<T> {
    pub fn new(
        color: Color,
        key: T,
        left: Option<Rc<RefCell<Self>>>,
        right: Option<Rc<RefCell<Self>>>,
    ) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(Self {
            color,
            key,
            left,
            right,
            p: Weak::new(),
        }));

        {
            let result_ref = result.borrow();

            if let Some(left) = &result_ref.left {
                left.borrow_mut().p = Rc::downgrade(&result);
            }

            if let Some(right) = &result_ref.right {
                right.borrow_mut().p = Rc::downgrade(&result);
            }
        }

        result
    }

    pub fn new_leaf(color: Color, key: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            color,
            key,
            left: None,
            right: None,
            p: Weak::new(),
        }))
    }
}

impl<T: PartialEq> PartialEq for RedBlackTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.key == other.key && self.left == other.left && self.right == other.right
    }
}

impl<T: Eq> Eq for RedBlackTreeNode<T> {}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Color, RedBlackTreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn check_valid_tree<T>(tree: &Option<Rc<RefCell<RedBlackTreeNode<T>>>>) {
        fn check_parents<T>(
            node: &Option<Rc<RefCell<RedBlackTreeNode<T>>>>,
            parent: &Rc<RefCell<RedBlackTreeNode<T>>>,
        ) {
            if let Some(node_rc) = node {
                let node_ref = node_rc.borrow();

                assert!(Rc::ptr_eq(&node_ref.p.upgrade().unwrap(), parent));

                check_parents(&node_ref.left, node_rc);
                check_parents(&node_ref.right, node_rc);
            }
        }

        if let Some(node_rc) = tree {
            let node_ref = node_rc.borrow();

            assert!(node_ref.p.upgrade().is_none());

            check_parents(&node_ref.left, node_rc);
            check_parents(&node_ref.right, node_rc);
        }
    }

    fn check_black_children<T: Ord>(node: &RedBlackTreeNode<T>) -> usize {
        let left_black_height = check_node(&node.left);
        let right_black_height = check_node(&node.right);

        assert_eq!(left_black_height, right_black_height);

        left_black_height + 1
    }

    #[allow(clippy::option_if_let_else)] // False positive.
    fn check_black_node<T: Ord>(node: &Option<Rc<RefCell<RedBlackTreeNode<T>>>>) -> usize {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();

            assert_eq!(node_ref.color, Color::Black);

            check_black_children(&node_ref)
        } else {
            0
        }
    }

    #[allow(clippy::option_if_let_else)] // False positive.
    fn check_node<T: Ord>(node: &Option<Rc<RefCell<RedBlackTreeNode<T>>>>) -> usize {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();

            match node_ref.color {
                Color::Red => {
                    let left_black_height = check_black_node(&node_ref.left);
                    let right_black_height = check_black_node(&node_ref.right);

                    assert_eq!(left_black_height, right_black_height);

                    left_black_height
                }
                Color::Black => check_black_children(&node_ref),
            }
        } else {
            0
        }
    }

    pub fn check_valid_red_black_tree<T: Ord>(tree: &Option<Rc<RefCell<RedBlackTreeNode<T>>>>) {
        check_valid_tree(tree);

        if let Some(node) = tree {
            let node_ref = node.borrow();

            assert!(node_ref.p.upgrade().is_none());
            assert_eq!(node_ref.color, Color::Black);

            check_black_children(&node_ref);
        }
    }
}
