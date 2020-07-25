use super::section_18_1_definition_of_b_trees::Node;
use std::borrow::Borrow;
use std::mem;

fn merge_into_left<K, V>(left: &mut Node<K, V>, middle: (K, V), right: &mut Node<K, V>) {
    left.data.push(middle);
    left.data.extend(right.data.drain(..));
    left.children.extend(right.children.drain(..));
}

fn borrow_into_left<K, V>(left: &mut Node<K, V>, middle: &mut (K, V), right: &mut Node<K, V>) {
    left.data.push(mem::replace(middle, right.data.remove(0)));

    if !right.children.is_empty() {
        left.children.push(right.children.remove(0));
    }
}

fn borrow_into_right<K, V>(left: &mut Node<K, V>, middle: &mut (K, V), right: &mut Node<K, V>) {
    right.data.insert(0, mem::replace(middle, left.data.pop().unwrap()));

    if let Some(left_child) = left.children.pop() {
        right.children.insert(0, left_child);
    }
}

fn delete_min<K: Borrow<Q>, V, Q: Ord + ?Sized>(node: &mut Node<K, V>, t: usize) -> (K, V) {
    if let [first_child, succeeding_child, ..] = node.children.as_mut_slice() {
        if first_child.data.len() < t {
            if succeeding_child.data.len() < t {
                merge_into_left(first_child, node.data.remove(0), succeeding_child);
                node.children.remove(1);

                delete_min(node.children.first_mut().unwrap(), t)
            } else {
                borrow_into_left(first_child, &mut node.data[0], succeeding_child);

                delete_min(first_child, t)
            }
        } else {
            delete_min(first_child, t)
        }
    } else {
        node.data.remove(0)
    }
}

fn delete_max<K: Borrow<Q>, V, Q: Ord + ?Sized>(node: &mut Node<K, V>, t: usize) -> (K, V) {
    if let [.., predeceding_child, last_child] = node.children.as_mut_slice() {
        if last_child.data.len() < t {
            if predeceding_child.data.len() < t {
                merge_into_left(predeceding_child, node.data.pop().unwrap(), last_child);
                node.children.pop();

                delete_max(node.children.last_mut().unwrap(), t)
            } else {
                borrow_into_right(predeceding_child, node.data.last_mut().unwrap(), last_child);

                delete_max(last_child, t)
            }
        } else {
            delete_max(last_child, t)
        }
    } else {
        node.data.pop().unwrap()
    }
}

fn b_tree_delete_i<K: Borrow<Q>, V, Q: Ord + ?Sized>(node: &mut Node<K, V>, t: usize, i: usize) -> (K, V) {
    if let Some([left_child, right_child, ..]) = node.children.get_mut(i..) {
        if left_child.data.len() < t {
            if right_child.data.len() < t {
                let child_data_index = left_child.data.len();

                merge_into_left(left_child, node.data.remove(i), right_child);
                node.children.remove(i + 1);

                b_tree_delete_i(&mut node.children[i], t, child_data_index)
            } else {
                mem::replace(&mut node.data[i], delete_min(right_child, t))
            }
        } else {
            mem::replace(&mut node.data[i], delete_max(left_child, t))
        }
    } else {
        node.data.remove(i)
    }
}

fn b_tree_delete_helper<K: Borrow<Q>, V, Q: Ord + ?Sized>(node: &mut Node<K, V>, t: usize, key: &Q) -> Option<V> {
    match node.data.binary_search_by(|(k, _)| k.borrow().cmp(key)) {
        Ok(i) => Some(b_tree_delete_i(node, t, i).1),
        Err(i) => {
            if let Some(child) = node.children.get_mut(i) {
                if child.data.len() < t {
                    if i == 0 {
                        match node.children.as_mut_slice() {
                            [child, right_child, ..] => {
                                if right_child.data.len() < t {
                                    merge_into_left(child, node.data.remove(0), right_child);
                                    node.children.remove(1);

                                    b_tree_delete_helper(&mut node.children[0], t, key)
                                } else {
                                    borrow_into_left(child, &mut node.data[0], right_child);

                                    b_tree_delete_helper(child, t, key)
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match &mut node.children[i - 1..] {
                            [left_child, child, right_child, ..] => {
                                if left_child.data.len() < t {
                                    if right_child.data.len() < t {
                                        merge_into_left(left_child, node.data.remove(0), child);
                                        node.children.remove(i);

                                        b_tree_delete_helper(&mut node.children[i - 1], t, key)
                                    } else {
                                        borrow_into_left(child, &mut node.data[i], right_child);

                                        b_tree_delete_helper(child, t, key)
                                    }
                                } else {
                                    borrow_into_right(left_child, &mut node.data[i - 1], child);

                                    b_tree_delete_helper(child, t, key)
                                }
                            }
                            [left_child, child] => {
                                if left_child.data.len() < t {
                                    merge_into_left(left_child, node.data.remove(0), child);
                                    node.children.pop();

                                    b_tree_delete_helper(node.children.last_mut().unwrap(), t, key)
                                } else {
                                    borrow_into_right(left_child, &mut node.data[i - 1], child);

                                    b_tree_delete_helper(child, t, key)
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                } else {
                    b_tree_delete_helper(child, t, key)
                }
            } else {
                None
            }
        }
    }
}

pub fn b_tree_delete<K: Borrow<Q>, V, Q: Ord + ?Sized>(node: &mut Node<K, V>, t: usize, key: &Q) -> Option<V> {
    let result = b_tree_delete_helper(node, t, key);

    if node.children.len() == 1 {
        *node = node.children.pop().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::section_18_1_definition_of_b_trees::tests::make_node;
    use super::{b_tree_delete, delete_max, delete_min};

    #[test]
    fn test_delete_min() {
        let test_cases = [
            ((make_node!(2 => 3, 5 => 7), 2), ((2, 3), make_node!(5 => 7))),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2),
                ((2, 3), make_node!(5 => 7, 11 => 13)),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29)), 2),
                ((2, 3), make_node!((5 => 7, 11 => 13), 17 => 19, (23 => 29))),
            ),
            (
                (
                    make_node!((2 => 3), 5 => 7, (11 => 13, 17 => 19), 23 => 29, (31 => 37)),
                    2,
                ),
                ((2, 3), make_node!((5 => 7), 11 => 13, (17 => 19), 23 => 29, (31 => 37))),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13)),
                        17 => 19,
                        ((23 => 29), 31 => 37, (41 => 43), 47 => 53, (59 => 61))
                    ),
                    2,
                ),
                (
                    (2, 3),
                    make_node!(
                        ((5 => 7, 11 => 13), 17 => 19, (23 => 29)),
                        31 => 37,
                        ((41 => 43), 47 => 53, (59 => 61))
                    ),
                ),
            ),
            (
                (
                    make_node!((2 => 3, 5 => 7), 11 => 13, (17 => 19), 23 => 29, (31 => 37)),
                    2,
                ),
                ((2, 3), make_node!((5 => 7), 11 => 13, (17 => 19), 23 => 29, (31 => 37))),
            ),
        ];

        for ((mut node, t), (expected_result, expected_node)) in test_cases.iter().cloned() {
            assert_eq!(delete_min(&mut node, t), expected_result);
            assert_eq!(node, expected_node);
        }
    }

    #[test]
    fn test_delete_max() {
        let test_cases = [
            ((make_node!(2 => 3, 5 => 7), 2), ((5, 7), make_node!(2 => 3))),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2),
                ((11, 13), make_node!(2 => 3, 5 => 7)),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29)), 2),
                ((23, 29), make_node!((2 => 3), 5 => 7, (11 => 13, 17 => 19))),
            ),
            (
                (
                    make_node!((2 => 3), 5 => 7, (11 => 13, 17 => 19), 23 => 29, (31 => 37)),
                    2,
                ),
                ((31, 37), make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29))),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29)),
                        31 => 37,
                        ((41 => 43), 47 => 53, (59 => 61))
                    ),
                    2,
                ),
                (
                    (59, 61),
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13)),
                        17 => 19,
                        ((23 => 29), 31 => 37, (41 => 43, 47 => 53))
                    ),
                ),
            ),
            (
                (
                    make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29, 31 => 37)),
                    2,
                ),
                ((31, 37), make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29))),
            ),
        ];

        for ((mut node, t), (expected_result, expected_node)) in test_cases.iter().cloned() {
            assert_eq!(delete_max(&mut node, t), expected_result);
            assert_eq!(node, expected_node);
        }
    }

    #[test]
    fn test_b_tree_delete() {
        let trees = [
            make_node!((('A' => 2, 'B' => 3),
                        'C' => 5,
                        ('D' => 7, 'E' => 11, 'F' => 13),
                        'G' => 17,
                        ('J' => 19, 'K' => 23, 'L' => 29),
                        'M' => 31,
                        ('N' => 37, 'O' => 41)),
                       'P' => 43,
                       (('Q' => 47, 'R' => 53, 'S' => 59),
                        'T' => 61,
                        ('U' => 67, 'V' => 71),
                        'X' => 73,
                        ('Y' => 79, 'Z' => 83))),
            make_node!((('A' => 2, 'B' => 3),
                        'C' => 5,
                        ('D' => 7, 'E' => 11),
                        'G' => 17,
                        ('J' => 19, 'K' => 23, 'L' => 29),
                        'M' => 31,
                        ('N' => 37, 'O' => 41)),
                       'P' => 43,
                       (('Q' => 47, 'R' => 53, 'S' => 59),
                        'T' => 61,
                        ('U' => 67, 'V' => 71),
                        'X' => 73,
                        ('Y' => 79, 'Z' => 83))),
            make_node!((('A' => 2, 'B' => 3),
                        'C' => 5,
                        ('D' => 7, 'E' => 11),
                        'G' => 17,
                        ('J' => 19, 'K' => 23),
                        'L' => 29,
                        ('N' => 37, 'O' => 41)),
                       'P' => 43,
                       (('Q' => 47, 'R' => 53, 'S' => 59),
                        'T' => 61,
                        ('U' => 67, 'V' => 71),
                        'X' => 73,
                        ('Y' => 79, 'Z' => 83))),
            make_node!((('A' => 2, 'B' => 3),
                        'C' => 5,
                        ('D' => 7, 'E' => 11, 'J' => 19, 'K' => 23),
                        'L' => 29,
                        ('N' => 37, 'O' => 41)),
                       'P' => 43,
                       (('Q' => 47, 'R' => 53, 'S' => 59),
                        'T' => 61,
                        ('U' => 67, 'V' => 71),
                        'X' => 73,
                        ('Y' => 79, 'Z' => 83))),
            make_node!(('A' => 2, 'B' => 3),
                       'C' => 5,
                       ('E' => 11, 'J' => 19, 'K' => 23),
                       'L' => 29,
                       ('N' => 37, 'O' => 41),
                       'P' => 43,
                       ('Q' => 47, 'R' => 53, 'S' => 59),
                       'T' => 61,
                       ('U' => 67, 'V' => 71),
                       'X' => 73,
                       ('Y' => 79, 'Z' => 83)),
            make_node!(('A' => 2, 'C' => 5),
                       'E' => 11,
                       ('J' => 19, 'K' => 23),
                       'L' => 29,
                       ('N' => 37, 'O' => 41),
                       'P' => 43,
                       ('Q' => 47, 'R' => 53, 'S' => 59),
                       'T' => 61,
                       ('U' => 67, 'V' => 71),
                       'X' => 73,
                       ('Y' => 79, 'Z' => 83)),
        ];

        let test_cases = [
            ((trees[0].clone(), 3, 'F'), (Some(13), trees[1].clone())),
            ((trees[1].clone(), 3, 'M'), (Some(31), trees[2].clone())),
            ((trees[2].clone(), 3, 'G'), (Some(17), trees[3].clone())),
            ((trees[3].clone(), 3, 'D'), (Some(7), trees[4].clone())),
            ((trees[4].clone(), 3, 'B'), (Some(3), trees[5].clone())),
        ];

        for ((mut node, t, key), (expected_result, expected_node)) in test_cases.iter().cloned() {
            assert_eq!(b_tree_delete(&mut node, t, &key), expected_result);
            assert_eq!(node, expected_node);
        }
    }
    #[test]
    fn test_b_tree_delete_extra() {
        let test_cases = [
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13, 17 => 19)), 2, 5),
                (Some(7), make_node!((2 => 3), 11 => 13, (17 => 19))),
            ),
            (
                (make_node!((2 => 3), 5 => 7,(11 => 13), 17 => 19, (23 => 29)), 2, 11),
                (Some(13), make_node!((2 => 3, 5 => 7), 17 => 19, (23 => 29))),
            ),
            (
                (
                    make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29, 31 => 37)),
                    2,
                    11,
                ),
                (Some(13), make_node!((2 => 3), 5 => 7, (17 => 19), 23 => 29, (31 => 37))),
            ),
            (
                (
                    make_node!((2 => 3, 5 => 7), 11 => 13, (17 => 19), 23 => 29, (31 => 37)),
                    2,
                    17,
                ),
                (Some(19), make_node!((2 => 3), 5 => 7, (11 => 13), 23 => 29, (31 => 37))),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13)), 2, 11),
                (Some(13), make_node!(2 => 3, 5 => 7)),
            ),
            (
                (make_node!((2 => 3, 5 => 7), 11 => 13, (17 => 19)), 2, 17),
                (Some(19), make_node!((2 => 3), 5 => 7, (11 => 13))),
            ),
            ((make_node!(), 2, 2), (None, make_node!())),
        ];

        for ((mut node, t, key), (expected_result, expected_node)) in test_cases.iter().cloned() {
            assert_eq!(b_tree_delete(&mut node, t, &key), expected_result);
            assert_eq!(node, expected_node);
        }
    }
}
