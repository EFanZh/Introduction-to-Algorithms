use std::mem::{size_of_val, swap};

struct Node<T> {
    value: T,
    children: Option<(usize, usize)>, // The left child must have the same value as this node
}

fn floor_log2(x: usize) -> usize {
    8 * size_of_val(&x) - 1 - x.leading_zeros() as usize
}

pub fn second_smallest<T: Ord>(a: &[T]) -> &T {
    let mut memory = Vec::with_capacity(a.len() * 2 + floor_log2(a.len() - 1) - 1);

    fn make_node<'a, T>(memory: &mut Vec<Node<&'a T>>, value: &'a T, children: Option<(usize, usize)>) -> usize {
        let handle = memory.len();

        memory.push(Node { value, children });

        handle
    }

    let mut b = a.iter().map(|x| make_node(&mut memory, x, None)).collect::<Vec<_>>();
    let mut temp = Vec::new();

    while b.len() > 1 {
        for maybe_pair in b.chunks(2) {
            if let [first, second] = *maybe_pair {
                let first_value = memory[first].value;
                let second_value = memory[second].value;

                if second_value < first_value {
                    temp.push(make_node(&mut memory, second_value, Some((second, first))));
                } else {
                    temp.push(make_node(&mut memory, first_value, Some((first, second))));
                }
            } else {
                temp.extend_from_slice(maybe_pair);
            }
        }

        swap(&mut b, &mut temp);
        temp.clear();
    }

    let (mut min_node_handle, right) = memory[b[0]].children.unwrap();
    let mut second_smallest_value = memory[right].value;

    while let Some((left, right)) = memory[min_node_handle].children {
        let right_value = memory[right].value;

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
