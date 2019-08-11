use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod exercises;

pub struct SimpleBinaryTreeNode<T> {
    pub key: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl<T> SimpleBinaryTreeNode<T> {
    pub fn new(key: T, left: Option<Box<Self>>, right: Option<Box<Self>>) -> Box<Self> {
        Box::new(Self { key, left, right })
    }

    pub fn new_leaf(key: T) -> Box<Self> {
        Box::new(Self {
            key,
            left: None,
            right: None,
        })
    }
}

#[macro_export]
macro_rules! make_simple_tree {
    (()) => {
        None
    };
    (($x:expr, $l:tt, $r:tt)) => {
        Some(SimpleBinaryTreeNode::new(
            $x,
            make_simple_tree!($l),
            make_simple_tree!($r),
        ))
    };
    ($x:expr) => {
        Some(SimpleBinaryTreeNode::new_leaf($x))
    };
}

pub struct BinaryTreeNode<T> {
    key: T,
    p: RefCell<Option<Weak<Self>>>,
    left: Option<Rc<Self>>,
    right: Option<Rc<Self>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(key: T, left: Option<Rc<Self>>, right: Option<Rc<Self>>) -> Rc<Self> {
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

    pub fn new_leaf(key: T) -> Rc<Self> {
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

    pub fn get_parent(&self) -> Option<Rc<Self>> {
        self.p.borrow().as_ref().map(|weak| weak.upgrade().unwrap())
    }

    pub fn get_left_child(&self) -> &Option<Rc<Self>> {
        &self.left
    }

    pub fn get_right_child(&self) -> &Option<Rc<Self>> {
        &self.right
    }
}

#[macro_export]
macro_rules! make_tree {
    (()) => {
        None
    };
    (($x:expr, $l:tt, $r:tt)) => {
        Some(BinaryTreeNode::new($x, make_tree!($l), make_tree!($r)))
    };
    ($x:expr) => {
        Some(BinaryTreeNode::new_leaf($x))
    };
}

pub struct UnboundedBranchingTreeNode<T> {
    key: T,
    p: RefCell<Option<Weak<Self>>>,
    left_child: Option<Rc<Self>>,
    right_sibling: Option<Rc<Self>>,
}

impl<T> UnboundedBranchingTreeNode<T> {
    pub fn new(key: T, left_child: Option<Rc<Self>>, right_sibling: Option<Rc<Self>>) -> Rc<Self> {
        let result = Rc::new(Self {
            key,
            p: RefCell::new(None),
            left_child,
            right_sibling,
        });

        let mut child = &result.left_child;

        while let Some(child_ref) = child {
            *child_ref.p.borrow_mut() = Some(Rc::downgrade(&result));

            child = &child_ref.right_sibling;
        }

        result
    }

    pub fn new_leaf(key: T) -> Rc<Self> {
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

    pub fn get_parent(&self) -> Option<Rc<Self>> {
        self.p.borrow().as_ref().map(|weak| weak.upgrade().unwrap())
    }

    pub fn get_left_child(&self) -> &Option<Rc<Self>> {
        &self.left_child
    }

    pub fn get_right_sibling(&self) -> &Option<Rc<Self>> {
        &self.right_sibling
    }
}

#[macro_export]
macro_rules! make_unbounded_branching_tree {
    (()) => {
        None
    };
    (($x:expr, $l:tt, $r:tt)) => {
        Some(UnboundedBranchingTreeNode::new(
            $x,
            make_unbounded_branching_tree!($l),
            make_unbounded_branching_tree!($r),
        ))
    };
    ($x:expr) => {
        Some(UnboundedBranchingTreeNode::new_leaf($x))
    };
}

#[cfg(test)]
mod tests {
    use super::{BinaryTreeNode, SimpleBinaryTreeNode};

    #[test]
    fn test_make_simple_tree() {
        let tree_1: Option<Box<SimpleBinaryTreeNode<i32>>> = make_simple_tree![()];

        assert!(tree_1.is_none());

        let tree_2 = make_simple_tree![4];

        assert_eq!(tree_2.unwrap().key, 4);

        let tree_3 = make_simple_tree![(1, 2, 3)].unwrap();

        assert_eq!(tree_3.key, 1);
        assert_eq!(tree_3.left.unwrap().key, 2);
        assert_eq!(tree_3.right.unwrap().key, 3);

        let tree_4 = make_simple_tree![(1, (), 3)].unwrap();

        assert_eq!(tree_4.key, 1);
        assert!(tree_4.left.is_none());
        assert_eq!(tree_4.right.unwrap().key, 3);

        let tree_5 = make_simple_tree![(1, 2, ())].unwrap();

        assert_eq!(tree_5.key, 1);
        assert_eq!(tree_5.left.unwrap().key, 2);
        assert!(tree_5.right.is_none());

        let tree_5 = make_simple_tree![(1, 2, (3, 4, 5))].unwrap();

        assert_eq!(tree_5.key, 1);
        assert_eq!(tree_5.left.unwrap().key, 2);

        let tree_5_right = tree_5.right.unwrap();

        assert_eq!(tree_5_right.key, 3);
        assert_eq!(tree_5_right.left.unwrap().key, 4);
        assert_eq!(tree_5_right.right.unwrap().key, 5);
    }

    #[test]
    fn test_make_tree() {
        let tree_1: Option<Box<BinaryTreeNode<i32>>> = make_tree![()];

        assert!(tree_1.is_none());

        let tree_2 = make_tree![4];

        assert_eq!(tree_2.unwrap().key, 4);

        let tree_3 = make_tree![(1, 2, 3)].unwrap();

        assert_eq!(tree_3.key, 1);
        assert_eq!(tree_3.left.as_ref().unwrap().key, 2);
        assert_eq!(tree_3.right.as_ref().unwrap().key, 3);

        let tree_4 = make_tree![(1, (), 3)].unwrap();

        assert_eq!(tree_4.key, 1);
        assert!(tree_4.left.is_none());
        assert_eq!(tree_4.right.as_ref().unwrap().key, 3);

        let tree_5 = make_tree![(1, 2, ())].unwrap();

        assert_eq!(tree_5.key, 1);
        assert_eq!(tree_5.left.as_ref().unwrap().key, 2);
        assert!(tree_5.right.is_none());

        let tree_5 = make_tree![(1, 2, (3, 4, 5))].unwrap();

        assert_eq!(tree_5.key, 1);
        assert_eq!(tree_5.left.as_ref().unwrap().key, 2);

        let tree_5_right = tree_5.right.as_ref().unwrap();

        assert_eq!(tree_5_right.key, 3);
        assert_eq!(tree_5_right.left.as_ref().unwrap().key, 4);
        assert_eq!(tree_5_right.right.as_ref().unwrap().key, 5);
    }
}
