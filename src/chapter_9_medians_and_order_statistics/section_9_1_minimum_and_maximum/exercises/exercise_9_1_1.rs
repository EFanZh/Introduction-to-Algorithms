use std::mem::{size_of_val, swap};

struct Node<T> {
    value: T,
    children: Option<(usize, usize)>, // The left child must have the same value as this node
}

impl<T> Node<T> {
    fn new(value: T, children: Option<(usize, usize)>) -> Node<T> {
        Node { value, children }
    }
}

fn floor_log2(x: usize) -> usize {
    8 * size_of_val(&x) - 1 - x.leading_zeros() as usize
}

struct Region<T> {
    memory: Vec<T>,
}

impl<T> Region<T> {
    fn with_capacity(capacity: usize) -> Region<T> {
        Region {
            memory: Vec::with_capacity(capacity),
        }
    }

    fn add(&mut self, value: T) -> usize {
        let handle = self.memory.len();

        self.memory.push(value);

        handle
    }

    fn get(&self, handle: usize) -> &T {
        &self.memory[handle]
    }
}

pub fn second_smallest<T: Ord>(a: &[T]) -> &T {
    let mut node_region = Region::with_capacity(a.len() * 2 + floor_log2(a.len() - 1) - 1);

    let mut b = a
        .iter()
        .map(|x| node_region.add(Node::new(x, None)))
        .collect::<Vec<_>>();

    let mut temp = Vec::new();

    while b.len() > 1 {
        for maybe_pair in b.chunks(2) {
            if let [first, second] = *maybe_pair {
                let first_value = node_region.get(first).value;
                let second_value = node_region.get(second).value;

                if second_value < first_value {
                    temp.push(node_region.add(Node::new(second_value, Some((second, first)))));
                } else {
                    temp.push(node_region.add(Node::new(first_value, Some((first, second)))));
                }
            } else {
                temp.extend_from_slice(maybe_pair);
            }
        }

        swap(&mut b, &mut temp);
        temp.clear();
    }

    let (mut min_node_handle, right) = node_region.get(b[0]).children.unwrap();
    let mut second_smallest_value = node_region.get(right).value;

    while let Some((left, right)) = node_region.get(min_node_handle).children {
        let right_value = node_region.get(right).value;

        if right_value < second_smallest_value {
            second_smallest_value = right_value;
        }

        min_node_handle = left;
    }

    second_smallest_value
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::loop_on_all_unordered_sequences;
    use super::{floor_log2, second_smallest};

    #[test]
    fn test_floor_log2() {
        assert_eq!(floor_log2(1), 0);
        assert_eq!(floor_log2(2), 1);
        assert_eq!(floor_log2(3), 1);
        assert_eq!(floor_log2(4), 2);
        assert_eq!(floor_log2(5), 2);
        assert_eq!(floor_log2(6), 2);
        assert_eq!(floor_log2(7), 2);
        assert_eq!(floor_log2(8), 3);
        assert_eq!(floor_log2(9), 3);
    }

    #[test]
    fn test_second_smallest() {
        loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            if let Some(expected_value) = sorted_sequence.get(1) {
                assert_eq!(second_smallest(sequence), expected_value);
            }
        });
    }
}
