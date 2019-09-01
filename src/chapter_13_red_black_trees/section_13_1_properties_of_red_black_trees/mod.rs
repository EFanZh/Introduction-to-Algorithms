use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(PartialEq, Eq, Debug)]
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

    pub fn new_leaf(color: Color, key: T) -> Rc<Self> {
        Rc::new(Self {
            color,
            key,
            left: None,
            right: None,
            p: Weak::new(),
        })
    }
}

impl<T: PartialEq> PartialEq for RedBlackTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.key == other.key && self.left == other.left && self.right == other.right
    }
}

impl<T: Eq> Eq for RedBlackTreeNode<T> {}
