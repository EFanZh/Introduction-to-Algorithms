use super::super::super::section_18_1_definition_of_b_trees::Node;
use super::super::{b_tree_create, b_tree_insert};

pub fn get_tree() -> Node<char, ()> {
    let mut result = b_tree_create();

    for key in [
        'F', 'S', 'Q', 'K', 'C', 'L', 'H', 'T', 'V', 'W', 'M', 'R', 'N', 'P', 'A', 'B', 'X', 'Y', 'D', 'Z', 'E',
    ]
    .iter()
    .copied()
    {
        b_tree_insert(&mut result, 2, key, ());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::super::super::super::section_18_1_definition_of_b_trees::tests::make_node;
    use super::get_tree;

    #[test]
    fn test_get_tree() {
        let result = get_tree();

        assert_eq!(
            result,
            make_node!(
                (
                    ('A' => ()),
                    'B' => (),
                    ('C' => (), 'D' => (), 'E' => ()),
                    'F' => (),
                    ('H' => ())
                ),
                'K' => (),
                (
                    ('L' => ()),
                    'M' => (),
                    ('N' => (), 'P' => ())
                ),
                'Q' => (),
                (
                    ('R' => (), 'S' => ()),
                    'T' => (),
                    ('V' => ()),
                    'W' => (),
                    ('X' => (), 'Y' => (), 'Z' => ())
                )
            )
        );
    }
}
