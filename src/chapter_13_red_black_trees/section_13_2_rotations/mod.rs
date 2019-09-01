use super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;

// Left-Rotate(T, x)
//
//  1  y = x.right           // set y
//  2  x.right = y.left      // turn y’s left subtree into x’s right subtree
//  3  if y.left ≠ T.nil
//  4  y.left.p = x
//  5      y.p = x.p         // link x’s parent to y
//  6  if x.p == T.nil
//  7      T.root = y
//  8  elseif x == x.p.left
//  9      x.p.left = y
// 10  else x.p.right = y
// 11  y.left = x            // put x on y’s left
// 12  x.p = y

pub fn left_rotate<T>(x: &mut Option<Box<RedBlackTreeNode<T>>>) {
    let mut x_node = x.take().unwrap();
    let mut y_node = x_node.right.take().unwrap();

    x_node.right = y_node.left.take();
    y_node.left = Some(x_node);

    *x = Some(y_node);
}

#[cfg(test)]
mod tests {
    use super::super::section_13_1_properties_of_red_black_trees::RedBlackTreeNode;
    use super::left_rotate;

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
    fn test_left_rotate() {
        let mut tree = make_node(
            2,
            make_node(1, None, None),
            make_node(4, make_node(3, None, None), make_node(5, None, None)),
        );

        left_rotate(&mut tree);

        assert_eq!(
            tree,
            make_node(
                4,
                make_node(2, make_node(1, None, None), make_node(3, None, None)),
                make_node(5, None, None)
            )
        );
    }
}
