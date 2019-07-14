pub mod exercises;

pub struct BinaryTreeNode<T> {
    pub key: T,
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(key: T, left: Option<Box<BinaryTreeNode<T>>>, right: Option<Box<BinaryTreeNode<T>>>) -> Self {
        Self { key, left, right }
    }

    pub fn new_boxed(key: T, left: Option<Box<BinaryTreeNode<T>>>, right: Option<Box<BinaryTreeNode<T>>>) -> Box<Self> {
        Box::new(Self::new(key, left, right))
    }

    pub fn leaf(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }

    pub fn boxed_leaf(key: T) -> Box<Self> {
        Box::new(Self::leaf(key))
    }
}

pub struct UnboundedBranchingTreeNode<T> {
    pub key: T,
    pub left_child: Option<Box<UnboundedBranchingTreeNode<T>>>,
    pub right_sibling: Option<Box<UnboundedBranchingTreeNode<T>>>,
}

impl<T> UnboundedBranchingTreeNode<T> {
    pub fn new(
        key: T,
        left_child: Option<Box<UnboundedBranchingTreeNode<T>>>,
        right_sibling: Option<Box<UnboundedBranchingTreeNode<T>>>,
    ) -> Self {
        Self {
            key,
            left_child,
            right_sibling,
        }
    }

    pub fn new_boxed(
        key: T,
        left_child: Option<Box<UnboundedBranchingTreeNode<T>>>,
        right_sibling: Option<Box<UnboundedBranchingTreeNode<T>>>,
    ) -> Box<Self> {
        Box::new(Self::new(key, left_child, right_sibling))
    }

    pub fn leaf(key: T) -> Self {
        Self {
            key,
            left_child: None,
            right_sibling: None,
        }
    }

    pub fn boxed_leaf(key: T) -> Box<Self> {
        Box::new(Self::leaf(key))
    }
}
