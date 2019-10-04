use std::rc::Rc;

mod insert;
mod remove;
mod search;

pub use insert::persistent_red_black_tree_insert;
pub use remove::persistent_red_black_tree_remove;
pub use search::persistent_red_black_tree_search;

// NodeContent.

#[derive(PartialEq, Eq, Debug)]
struct NodeContent<K, V> {
    key: Rc<K>,
    value: Rc<V>,
}

impl<K, V> NodeContent<K, V> {
    fn with_value(&self, value: Rc<V>) -> Self {
        Self {
            key: self.key.clone(),
            value,
        }
    }
}

impl<K, V> Clone for NodeContent<K, V> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

// RedNode.

#[derive(PartialEq, Eq, Debug)]
struct RedNode<K, V> {
    content: NodeContent<K, V>,
    left: RedBlackTree<K, V>,
    right: RedBlackTree<K, V>,
}

impl<K, V> RedNode<K, V> {
    fn new<L: Into<RedBlackTree<K, V>>, R: Into<RedBlackTree<K, V>>>(
        content: NodeContent<K, V>,
        left: L,
        right: R,
    ) -> Self {
        Self {
            content,
            left: left.into(),
            right: right.into(),
        }
    }

    fn new_rc<L: Into<RedBlackTree<K, V>>, R: Into<RedBlackTree<K, V>>>(
        content: NodeContent<K, V>,
        left: L,
        right: R,
    ) -> Rc<Self> {
        Rc::new(Self::new(content, left, right))
    }

    fn new_leaf(content: NodeContent<K, V>) -> Self {
        Self::new(content, None, None)
    }

    fn with_value_rc(&self, value: Rc<V>) -> Rc<Self> {
        Self::new_rc(self.content.with_value(value), self.left.clone(), self.right.clone())
    }

    fn with_black_color_rc(&self) -> Rc<BlackNode<K, V>> {
        BlackNode::new_rc(self.content.clone(), self.left.clone(), self.right.clone())
    }

    fn with_left_rc<L: Into<RedBlackTree<K, V>>>(&self, left: L) -> Rc<Self> {
        Self::new_rc(self.content.clone(), left, self.right.clone())
    }

    fn with_right_rc<R: Into<RedBlackTree<K, V>>>(&self, right: R) -> Rc<Self> {
        Self::new_rc(self.content.clone(), self.left.clone(), right)
    }
}

// BlackNode.

#[derive(PartialEq, Eq, Debug)]
enum RedOrBlackNode<K, V> {
    Red(Rc<RedNode<K, V>>),
    Black(RedBlackTree<K, V>),
}

impl<K, V> Clone for RedOrBlackNode<K, V> {
    fn clone(&self) -> Self {
        match self {
            Self::Red(node) => Self::Red(node.clone()),
            Self::Black(node) => Self::Black(node.clone()),
        }
    }
}

impl<K, V> From<Rc<RedNode<K, V>>> for RedOrBlackNode<K, V> {
    fn from(value: Rc<RedNode<K, V>>) -> Self {
        Self::Red(value)
    }
}

impl<K, V> From<Rc<BlackNode<K, V>>> for RedOrBlackNode<K, V> {
    fn from(value: Rc<BlackNode<K, V>>) -> Self {
        Self::Black(Some(value))
    }
}

impl<K, V> From<RedBlackTree<K, V>> for RedOrBlackNode<K, V> {
    fn from(value: RedBlackTree<K, V>) -> Self {
        Self::Black(value)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BlackNode<K, V> {
    content: NodeContent<K, V>,
    left: RedOrBlackNode<K, V>,
    right: RedOrBlackNode<K, V>,
}

impl<K, V> BlackNode<K, V> {
    fn new<L: Into<RedOrBlackNode<K, V>>, R: Into<RedOrBlackNode<K, V>>>(
        content: NodeContent<K, V>,
        left: L,
        right: R,
    ) -> Self {
        Self {
            content,
            left: left.into(),
            right: right.into(),
        }
    }

    fn new_rc<L: Into<RedOrBlackNode<K, V>>, R: Into<RedOrBlackNode<K, V>>>(
        content: NodeContent<K, V>,
        left: L,
        right: R,
    ) -> Rc<Self> {
        Rc::new(Self::new(content, left, right))
    }

    fn with_value(&self, value: Rc<V>) -> Self {
        Self::new(self.content.with_value(value), self.left.clone(), self.right.clone())
    }

    fn with_left<L: Into<RedOrBlackNode<K, V>>>(&self, left: L) -> Self {
        Self::new(self.content.clone(), left, self.right.clone())
    }

    fn with_left_rc<L: Into<RedOrBlackNode<K, V>>>(&self, left: L) -> Rc<Self> {
        Self::new_rc(self.content.clone(), left, self.right.clone())
    }

    fn with_right<R: Into<RedOrBlackNode<K, V>>>(&self, right: R) -> Self {
        Self::new(self.content.clone(), self.left.clone(), right)
    }

    fn with_right_rc<R: Into<RedOrBlackNode<K, V>>>(&self, right: R) -> Rc<Self> {
        Self::new_rc(self.content.clone(), self.left.clone(), right)
    }
}

type RedBlackTree<K, V> = Option<Rc<BlackNode<K, V>>>;

#[cfg(test)]
mod tests {
    use super::{BlackNode, NodeContent, RedBlackTree, RedNode, RedOrBlackNode};
    use std::rc::Rc;

    pub(super) fn red<K, V, L: Into<RedBlackTree<K, V>>, R: Into<RedBlackTree<K, V>>>(
        key: K,
        value: V,
        left: L,
        right: R,
    ) -> Rc<RedNode<K, V>> {
        RedNode::new_rc(
            NodeContent {
                key: key.into(),
                value: value.into(),
            },
            left,
            right,
        )
    }

    pub(super) fn red_leaf<K, V>(key: K, value: V) -> Rc<RedNode<K, V>> {
        red(key, value, None, None)
    }

    pub(super) fn black<K, V, L: Into<RedOrBlackNode<K, V>>, R: Into<RedOrBlackNode<K, V>>>(
        key: K,
        value: V,
        left: L,
        right: R,
    ) -> Rc<BlackNode<K, V>> {
        BlackNode::new_rc(
            NodeContent {
                key: key.into(),
                value: value.into(),
            },
            left,
            right,
        )
    }

    pub(super) fn black_leaf<K, V>(key: K, value: V) -> Rc<BlackNode<K, V>> {
        black(key, value, None, None)
    }
}
