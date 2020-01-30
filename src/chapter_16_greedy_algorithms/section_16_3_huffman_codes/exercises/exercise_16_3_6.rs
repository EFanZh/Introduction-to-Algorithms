use std::iter;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Node {
    Leaf { value: usize },
    Node { left: Box<Self>, right: Box<Self> },
}

fn decode_enumeration<I: IntoIterator<Item = bool>>(length: usize, bits: I) -> impl Iterator<Item = usize> {
    let num_internal_nodes = length - 1;
    let mut iter = bits.into_iter();
    let mut i = 0usize;

    iter::from_fn(move || loop {
        if let Some(index) = i.checked_sub(num_internal_nodes) {
            i = 0;

            return Some(index);
        } else if let Some(bit) = iter.next() {
            if bit {
                i = i * 2 + 2;
            } else {
                i = i * 2 + 1;
            }
        } else {
            return None;
        }
    })
}

fn get_enumeration_encoder(length: usize) -> Box<[Box<[bool]>]> {
    let num_internal_nodes = length - 1;

    (num_internal_nodes..)
        .take(length)
        .map(|mut i| {
            let mut code = iter::from_fn(move || {
                if i == 0 {
                    None
                } else {
                    let result = i;

                    i = (i - 1) / 2;

                    Some(result)
                }
            })
            .map(|i| i % 2 == 0)
            .collect::<Box<_>>();

            code.reverse();

            code
        })
        .collect()
}

fn encode_helper(node: &Node) -> Vec<bool> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    let mut values = Vec::new();
    let mut current_node = node;

    loop {
        match current_node {
            Node::Leaf { value } => {
                result.push(true);
                values.push(*value);

                if let Some(right) = stack.pop() {
                    current_node = right;
                } else {
                    break;
                }
            }
            Node::Node { left, right } => {
                result.push(false);
                stack.push(right);

                current_node = left;
            }
        }
    }

    let encoder = get_enumeration_encoder(values.len());

    for value in values {
        result.extend(encoder[value].iter().copied());
    }

    result
}

fn decode_helper<I: IntoIterator<Item = bool>>(values: I) -> Node {
    let mut result = Node::Leaf {
        value: usize::max_value(),
    };

    let mut iter = values.into_iter();
    let mut slots = Vec::new();
    let mut stack = Vec::new();
    let mut node_ref = &mut result;

    for value in iter.by_ref() {
        if value {
            slots.push(node_ref);

            if let Some(right) = stack.pop() {
                node_ref = right;
            } else {
                break;
            }
        } else {
            *node_ref = Node::Node {
                left: Box::new(Node::Leaf {
                    value: usize::max_value(),
                }),
                right: Box::new(Node::Leaf {
                    value: usize::max_value(),
                }),
            };

            if let Node::Node { left, right } = node_ref {
                stack.push(&mut *right);

                node_ref = &mut *left;
            } else {
                unreachable!();
            }
        }
    }

    let length = slots.len();

    for (slot, i) in slots.into_iter().zip(decode_enumeration(length, iter)) {
        if let Node::Leaf { value } = slot {
            *value = i;
        } else {
            unreachable!();
        }
    }

    result
}

fn compress_bits(bits: &[bool]) -> (Box<[u8]>, usize) {
    (
        bits.chunks(8)
            .map(|bits| {
                let mut result = 0;

                for (i, bit) in bits.iter().copied().enumerate() {
                    result |= (bit as u8) << i;
                }

                result
            })
            .collect(),
        bits.len(),
    )
}

fn decompress_bits<'a>(bits: &'a [u8], length: usize) -> impl 'a + Iterator<Item = bool> {
    bits.iter()
        .flat_map(|byte| (0..8).map(move |i| byte & (1 << i) != 0))
        .take(length)
}

pub fn encode_tree(tree: &Node) -> (Box<[u8]>, usize) {
    compress_bits(&encode_helper(tree))
}

pub fn decode_tree(data: &[u8], length: usize) -> Node {
    decode_helper(decompress_bits(data, length))
}

#[cfg(test)]
mod tests {
    use super::{decode_tree, encode_tree, Node};

    type TestCase = (Node, (Box<[u8]>, usize));

    fn make_tree(left: Node, right: Node) -> Node {
        Node::Node {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn make_leaf(value: usize) -> Node {
        Node::Leaf { value }
    }

    fn get_test_cases() -> Box<[TestCase]> {
        let test_cases = [
            (make_leaf(0), (&[0b_1_u8] as &[_], 1)),
            (make_tree(make_leaf(0), make_leaf(1)), (&[0b_10_110], 5)),
            (make_tree(make_leaf(1), make_leaf(0)), (&[0b_01_110], 5)),
            (
                make_tree(make_leaf(0), make_tree(make_leaf(1), make_leaf(2))),
                (&[0b_00_1_11010, 0b_10], 10),
            ),
            (
                make_tree(make_leaf(0), make_tree(make_leaf(2), make_leaf(1))),
                (&[0b_10_1_11010, 0b_00], 10),
            ),
            (
                make_tree(make_leaf(1), make_tree(make_leaf(0), make_leaf(2))),
                (&[0b_1_00_11010, 0b_10], 10),
            ),
            (
                make_tree(make_leaf(1), make_tree(make_leaf(2), make_leaf(0))),
                (&[0b_0_00_11010, 0b_1_1], 10),
            ),
            (
                make_tree(make_leaf(2), make_tree(make_leaf(0), make_leaf(1))),
                (&[0b_1_10_11010, 0b_00], 10),
            ),
            (
                make_tree(make_leaf(2), make_tree(make_leaf(1), make_leaf(0))),
                (&[0b_0_10_11010, 0b_1_0], 10),
            ),
            (
                make_tree(make_tree(make_leaf(0), make_leaf(1)), make_leaf(2)),
                (&[0b_00_1_11100, 0b_10], 10),
            ),
            (
                make_tree(make_tree(make_leaf(0), make_leaf(2)), make_leaf(1)),
                (&[0b_10_1_11100, 0b_00], 10),
            ),
            (
                make_tree(make_tree(make_leaf(1), make_leaf(0)), make_leaf(2)),
                (&[0b_1_00_11100, 0b_10], 10),
            ),
            (
                make_tree(make_tree(make_leaf(1), make_leaf(2)), make_leaf(0)),
                (&[0b_0_00_11100, 0b_1_1], 10),
            ),
            (
                make_tree(make_tree(make_leaf(2), make_leaf(0)), make_leaf(1)),
                (&[0b_1_10_11100, 0b_00], 10),
            ),
            (
                make_tree(make_tree(make_leaf(2), make_leaf(1)), make_leaf(0)),
                (&[0b_0_10_11100, 0b_1_0], 10),
            ),
            (
                make_tree(
                    make_leaf(2),
                    make_tree(
                        make_leaf(0),
                        make_tree(make_leaf(3), make_tree(make_leaf(1), make_leaf(4))),
                    ),
                ),
                (&[0b_10101010, 0b_000_10_11_1, 0b_100_01], 21),
                //        tree       3  0  2         4  1
            ),
        ];

        test_cases
            .iter()
            .map(|(tree, (data, length))| (tree.clone(), (data.iter().copied().collect(), *length)))
            .collect()
    }

    #[test]
    fn test_encode_tree() {
        for (tree, (expected_data, expected_length)) in get_test_cases().into_vec() {
            let (data, length) = encode_tree(&tree);

            assert_eq!(data, expected_data);
            assert_eq!(length, expected_length);
        }
    }

    #[test]
    fn test_decode_tree() {
        for (expected_tree, (data, length)) in get_test_cases().into_vec() {
            let tree = decode_tree(&data, length);

            assert_eq!(tree, expected_tree);
        }
    }
}
