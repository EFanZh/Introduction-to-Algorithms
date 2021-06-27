#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<K, V> {
    pub(super) data: Vec<(K, V)>,
    pub(super) children: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V> {
    #[must_use]
    pub fn internal(data: Vec<(K, V)>, children: Vec<Node<K, V>>) -> Self {
        assert_eq!(data.len() + 1, children.len());

        Self { data, children }
    }

    #[must_use]
    pub fn leaf(data: Vec<(K, V)>) -> Self {
        Self {
            data,
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::Node;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __make_btree_node {
        (($($first_child:tt)*) $(, $key:expr => $value:expr, ($($child:tt)*))+) => {
            $crate::chapter_18_basic_operations_on_b_trees::section_18_1_definition_of_b_trees::Node::internal(
                ::std::vec![$(($key, $value)),*],
                ::std::vec![
                    crate::__make_btree_node!($($first_child)*)
                    $(, crate::__make_btree_node!($($child)*))*
                ],
            )
        };
        ($($key:expr => $value:expr),*) => {
            $crate::chapter_18_basic_operations_on_b_trees::section_18_1_definition_of_b_trees::Node::leaf(
                ::std::vec![$(($key, $value)),*],
            )
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
            Node::internal(vec![(2, 3)], vec![Node::leaf(Vec::new()), Node::leaf(Vec::new())])
        );

        assert_eq!(
            make_node!((1 => 2), 2 => 3, (5 => 7)),
            Node::internal(vec![(2, 3)], vec![Node::leaf(vec![(1, 2)]), Node::leaf(vec![(5, 7)])])
        );
    }
}
