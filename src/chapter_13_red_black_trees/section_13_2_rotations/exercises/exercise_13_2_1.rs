use super::super::super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;

pub fn right_rotate<T>(y: &mut Option<Box<RedBlackTreeNode<T>>>) {
    let mut y_node = y.take().unwrap();
    let mut x_node = y_node.left.take().unwrap();

    y_node.left = x_node.right.take();
    x_node.right = Some(y_node);

    *y = Some(x_node);
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;
    use super::right_rotate;

    type Tree<T> = Option<Box<RedBlackTreeNode<T>>>;

    fn make_node<T>(key: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Some(Box::new(RedBlackTreeNode {
            is_red: false,
            key,
            left,
            right,
        }))
    }

    #[test]
    fn test_right_rotate() {
        let mut tree = make_node(
            4,
            make_node(2, make_node(1, None, None), make_node(3, None, None)),
            make_node(5, None, None),
        );

        right_rotate(&mut tree);

        assert_eq!(
            tree,
            make_node(
                2,
                make_node(1, None, None),
                make_node(4, make_node(3, None, None), make_node(5, None, None)),
            )
        );
    }
}
