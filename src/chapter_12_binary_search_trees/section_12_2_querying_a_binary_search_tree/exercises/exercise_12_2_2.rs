use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;

pub fn recursive_tree_minimum<T: Ord>(x: &SimpleBinaryTreeNode<T>) -> &SimpleBinaryTreeNode<T> {
    x.left.as_deref().map_or(x, recursive_tree_minimum)
}

pub fn recursive_tree_maximum<T: Ord>(x: &SimpleBinaryTreeNode<T>) -> &SimpleBinaryTreeNode<T> {
    x.right.as_deref().map_or(x, recursive_tree_maximum)
}

#[cfg(test)]
mod tests {
    use super::{recursive_tree_maximum, recursive_tree_minimum};
    use crate::make_simple_tree;

    #[test]
    fn test_recursive_tree_minimum() {
        assert_eq!(recursive_tree_minimum(&make_simple_tree![1].unwrap()).key, 1);
        assert_eq!(recursive_tree_minimum(&make_simple_tree![(5, 2, 8)].unwrap()).key, 2);
    }

    #[test]
    fn test_recursive_tree_maximum() {
        assert_eq!(recursive_tree_maximum(&make_simple_tree![1].unwrap()).key, 1);
        assert_eq!(recursive_tree_maximum(&make_simple_tree![(5, 2, 8)].unwrap()).key, 8);
    }
}
