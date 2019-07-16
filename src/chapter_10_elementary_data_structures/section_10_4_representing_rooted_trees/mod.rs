use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

pub mod exercises;

pub struct BinaryTreeNode<T> {
    key: T,
    p: RefCell<Option<Weak<BinaryTreeNode<T>>>>,
    left: Option<Rc<BinaryTreeNode<T>>>,
    right: Option<Rc<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new_boxed(key: T, left: Option<Rc<BinaryTreeNode<T>>>, right: Option<Rc<BinaryTreeNode<T>>>) -> Rc<Self> {
        let result = Rc::new(Self {
            key,
            p: RefCell::new(None),
            left,
            right,
        });

        if let Some(left) = &result.left {
            *left.p.borrow_mut() = Some(Rc::downgrade(&result));
        }

        if let Some(right) = &result.right {
            *right.p.borrow_mut() = Some(Rc::downgrade(&result));
        }

        result
    }

    pub fn boxed_leaf(key: T) -> Rc<Self> {
        Rc::new(Self {
            key,
            p: RefCell::new(None),
            left: None,
            right: None,
        })
    }

    pub fn get_key(&self) -> &T {
        &self.key
    }

    pub fn get_parent(&self) -> Ref<Option<Weak<BinaryTreeNode<T>>>> {
        self.p.borrow()
    }

    pub fn get_left_child(&self) -> &Option<Rc<BinaryTreeNode<T>>> {
        &self.left
    }

    pub fn get_right_child(&self) -> &Option<Rc<BinaryTreeNode<T>>> {
        &self.right
    }
}

pub struct UnboundedBranchingTreeNode<T> {
    key: T,
    p: RefCell<Option<Weak<UnboundedBranchingTreeNode<T>>>>,
    left_child: Option<Rc<UnboundedBranchingTreeNode<T>>>,
    right_sibling: Option<Rc<UnboundedBranchingTreeNode<T>>>,
}

impl<T> UnboundedBranchingTreeNode<T> {
    pub fn new_boxed(
        key: T,
        left_child: Option<Rc<UnboundedBranchingTreeNode<T>>>,
        right_sibling: Option<Rc<UnboundedBranchingTreeNode<T>>>,
    ) -> Rc<Self> {
        let result = Rc::new(Self {
            key,
            p: RefCell::new(None),
            left_child,
            right_sibling,
        });

        if let Some(left_child) = &result.left_child {
            *left_child.p.borrow_mut() = Some(Rc::downgrade(&result));
        }

        if let Some(right_sibling) = &result.right_sibling {
            *right_sibling.p.borrow_mut() = Some(Rc::downgrade(&result));
        }

        result
    }

    pub fn boxed_leaf(key: T) -> Rc<Self> {
        Rc::new(Self {
            key,
            p: RefCell::new(None),
            left_child: None,
            right_sibling: None,
        })
    }

    pub fn get_key(&self) -> &T {
        &self.key
    }

    pub fn get_parent(&self) -> Ref<Option<Weak<UnboundedBranchingTreeNode<T>>>> {
        self.p.borrow()
    }

    pub fn get_left_child(&self) -> &Option<Rc<UnboundedBranchingTreeNode<T>>> {
        &self.left_child
    }

    pub fn get_right_sibling(&self) -> &Option<Rc<UnboundedBranchingTreeNode<T>>> {
        &self.right_sibling
    }
}
