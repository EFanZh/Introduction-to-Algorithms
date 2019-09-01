#[derive(PartialEq, Eq, Debug)]
pub struct RedBlackTreeNode<T> {
    pub is_red: bool,
    pub key: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}
