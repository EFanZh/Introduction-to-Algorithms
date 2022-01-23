use crate::chapter_10_elementary_data_structures::section_10_4_representing_rooted_trees::SimpleBinaryTreeNode;

pub fn tree_insert<T: Ord>(x: &mut Option<Box<SimpleBinaryTreeNode<T>>>, z: Box<SimpleBinaryTreeNode<T>>) {
    if let Some(x_2) = x {
        if z.key < x_2.key {
            tree_insert(&mut x_2.left, z);
        } else {
            tree_insert(&mut x_2.right, z);
        }
    } else {
        *x = Some(z);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests;

    #[test]
    fn test_tree_insert() {
        tests::run_tree_insert_test_cases(super::tree_insert);
    }
}
