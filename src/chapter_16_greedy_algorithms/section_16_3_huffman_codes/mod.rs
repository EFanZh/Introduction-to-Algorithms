use crate::utilities::KeyValuePair;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub mod exercises;

#[derive(PartialEq, Eq, Debug)]
pub enum NodeContent<T> {
    Node {
        left: Box<TreeNode<T>>,
        right: Box<TreeNode<T>>,
    },
    Leaf {
        key: T,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode<T> {
    pub frequency: u64,
    pub content: NodeContent<T>,
}

// Huffman(C)
//
// 1  n = |C|
// 2  Q = C
// 3  for i = 1 to n - 1
// 4  allocate a new node z
// 5      z.left = x = Extract-Min(Q)
// 6      z.right = y = Extract-Min(Q)
// 7      z.freq = x.freq + y.freq
// 8      Insert(Q, z)
// 9  return Extract-Min(Q) // return the root of the tree

pub fn huffman<T, I: IntoIterator<Item = (T, u64)>>(c: I) -> TreeNode<T> {
    let mut q = c
        .into_iter()
        .map(|(v, f)| KeyValuePair::new(Reverse(f), NodeContent::Leaf { key: v }))
        .collect::<BinaryHeap<_>>();

    loop {
        let KeyValuePair {
            key: Reverse(fx),
            value: x,
        } = q.pop().unwrap();

        if let Some(KeyValuePair {
            key: Reverse(fy),
            value: y,
        }) = q.pop()
        {
            q.push(KeyValuePair::new(
                Reverse(fx + fy),
                NodeContent::Node {
                    left: Box::new(TreeNode {
                        frequency: fx,
                        content: x,
                    }),
                    right: Box::new(TreeNode {
                        frequency: fy,
                        content: y,
                    }),
                },
            ));
        } else {
            break TreeNode {
                frequency: fx,
                content: x,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{huffman, NodeContent, TreeNode};

    fn make_node<T>(frequency: u64, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode {
            frequency,
            content: NodeContent::Node {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    fn make_leaf<T>(frequency: u64, key: T) -> TreeNode<T> {
        TreeNode {
            frequency,
            content: NodeContent::Leaf { key },
        }
    }

    #[test]
    fn test_huffman() {
        let c = [('a', 45), ('b', 13), ('c', 12), ('d', 16), ('e', 9), ('f', 5)];
        let tree = huffman(c.iter().copied());

        let expected_tree = make_node(
            100,
            make_leaf(45, 'a'),
            make_node(
                55,
                make_node(25, make_leaf(12, 'c'), make_leaf(13, 'b')),
                make_node(
                    30,
                    make_node(14, make_leaf(5, 'f'), make_leaf(9, 'e')),
                    make_leaf(16, 'd'),
                ),
            ),
        );

        assert_eq!(tree, expected_tree);
    }
}
