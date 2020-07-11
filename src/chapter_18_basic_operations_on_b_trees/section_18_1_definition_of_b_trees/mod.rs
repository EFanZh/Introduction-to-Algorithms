#[derive(PartialEq, Eq, Clone, Debug)]
pub(super) struct LeafNode<K, V> {
    pub(super) data: Vec<(K, V)>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item<K, V> {
    pub(super) child: Box<Node<K, V>>,
    pub(super) key: K,
    pub(super) value: V,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub(super) struct InternalNode<K, V> {
    pub(super) data: Vec<Item<K, V>>,
    pub(super) last_child: Box<Node<K, V>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub(super) enum NodeImpl<K, V> {
    Internal(Box<InternalNode<K, V>>),
    Leaf(Box<LeafNode<K, V>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<K, V>(pub(super) NodeImpl<K, V>);

impl<K, V> Node<K, V> {
    pub fn internal(data: Vec<Item<K, V>>, last_child: Box<Node<K, V>>) -> Self {
        Self(NodeImpl::Internal(Box::new(InternalNode { data, last_child })))
    }

    pub fn leaf(data: Vec<(K, V)>) -> Self {
        Self(NodeImpl::Leaf(Box::new(LeafNode { data })))
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::{Item, Node};

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __make_btree_node {
        (@internal [$((($($saved_child:tt)*), $saved_key:expr, $saved_value:expr))*] ($($last_child:tt)*)) => {
            $crate::chapter_18_basic_operations_on_b_trees::section_18_1_definition_of_b_trees::Node::internal(
                ::std::vec![
                    $(
                        $crate::chapter_18_basic_operations_on_b_trees::section_18_1_definition_of_b_trees::Item {
                            child: ::std::boxed::Box::new($crate::__make_btree_node!($($saved_child)*)),
                            key: $saved_key,
                            value: $saved_value,
                        }
                    ),*
                ],
                ::std::boxed::Box::new($crate::__make_btree_node!($($last_child)*))
            )
        };
        (@internal [$($saved:tt)*] ($($child:tt)*), $key:expr => $value:expr, $($rest:tt)*) => {
            $crate::__make_btree_node!(
                @internal
                [
                    $($saved)*
                    (
                        ($($child)*),
                        $key,
                        $value
                    )
                ]
                $($rest)*
            )
        };
        (($($first_child:tt)*), $key:expr => $value:expr, $($rest:tt)*) => {
            $crate::__make_btree_node!(@internal [(($($first_child)*), $key, $value)] $($rest)*)
        };
        ($($key:expr => $value:expr),*) => {
            $crate::chapter_18_basic_operations_on_b_trees::section_18_1_definition_of_b_trees::Node::leaf(::std::vec![$(($key, $value)),*])
        };
    }

    pub use crate::__make_btree_node as make_node;

    #[test]
    fn test_make_node_leaf() {
        assert_eq!(make_node!(), Node::<i32, i32>::leaf(Vec::new()));
        assert_eq!(make_node!(2 => 3), Node::leaf(vec![(2, 3)]));

        assert_eq!(make_node!(2 => 3, 5 => 7), Node::leaf(vec![(2, 3), (5, 7)]));

        assert_eq!(
            make_node!(2 => 3, 5 => 7, 11 => 13),
            Node::leaf(vec![(2, 3), (5, 7), (11, 13)])
        );
    }

    #[test]
    fn test_make_node_internal() {
        assert_eq!(
            make_node!((), 2 => 3, ()),
            Node::internal(
                vec![Item {
                    child: Box::new(Node::leaf(Vec::new())),
                    key: 2,
                    value: 3
                }],
                Box::new(Node::leaf(Vec::new()))
            )
        );

        assert_eq!(
            make_node!((1 => 2), 2 => 3, (5 => 7)),
            Node::internal(
                vec![Item {
                    child: Box::new(Node::leaf(vec![(1, 2)])),
                    key: 2,
                    value: 3
                }],
                Box::new(Node::leaf(vec![(5, 7)]))
            )
        );
    }
}
