#[cfg(test)]
mod tests {
    use super::super::super::super::section_18_1_definition_of_b_trees::tests::make_node;
    use super::super::super::b_tree_delete;

    #[test]
    fn test_check_result() {
        let trees = [
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
            make_node!(('A' => 2, 'E' => 11, 'J' => 19, 'K' => 23),
                       'L' => 29,
                       ('N' => 37, 'O' => 41),
                       'P' => 43,
                       ('Q' => 47, 'R' => 53, 'S' => 59),
                       'T' => 61,
                       ('U' => 67, 'V' => 71),
                       'X' => 73,
                       ('Y' => 79, 'Z' => 83)),
            make_node!(('A' => 2, 'E' => 11, 'J' => 19, 'K' => 23),
                       'L' => 29,
                       ('N' => 37, 'O' => 41),
                       'Q' => 47,
                       ('R' => 53, 'S' => 59),
                       'T' => 61,
                       ('U' => 67, 'V' => 71),
                       'X' => 73,
                       ('Y' => 79, 'Z' => 83)),
            make_node!(('A' => 2, 'E' => 11, 'J' => 19, 'K' => 23),
                       'L' => 29,
                       ('N' => 37, 'O' => 41),
                       'Q' => 47,
                       ('R' => 53, 'S' => 59, 'T' => 61, 'U' => 67),
                       'X' => 73,
                       ('Y' => 79, 'Z' => 83)),
        ];

        let test_cases = [
            ((trees[0].clone(), 3, 'C'), (Some(5), trees[1].clone())),
            ((trees[1].clone(), 3, 'P'), (Some(43), trees[2].clone())),
            ((trees[2].clone(), 3, 'V'), (Some(71), trees[3].clone())),
        ];

        for ((mut node, t, key), (expected_result, expected_node)) in test_cases.iter().cloned() {
            assert_eq!(b_tree_delete(&mut node, t, &key), expected_result);
            assert_eq!(node, expected_node);
        }
    }
}
