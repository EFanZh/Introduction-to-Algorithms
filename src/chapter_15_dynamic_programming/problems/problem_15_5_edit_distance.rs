#[derive(PartialEq, Eq, Debug)]
pub enum Operation<T> {
    Copy,
    Replace(T),
    Delete,
    Insert(T),
    Twiddle,
    Kill,
}

pub fn transform<T: Clone>(source: &[T], operations: &[Operation<T>]) -> Box<[T]> {
    let mut result = vec![];
    let mut source_iter = source.iter();
    let mut operation_iter = operations.iter();

    while let Some(operation) = operation_iter.next() {
        match operation {
            Operation::Copy => result.push(source_iter.next().unwrap().clone()),
            Operation::Replace(value) => {
                source_iter.next().unwrap();
                result.push(value.clone())
            }
            Operation::Delete => {
                source_iter.next().unwrap();
            }
            Operation::Insert(value) => result.push(value.clone()),
            Operation::Twiddle => {
                let first = source_iter.next().unwrap();
                let second = source_iter.next().unwrap();

                result.push(second.clone());
                result.push(first.clone());
            }
            Operation::Kill => {
                assert!(operation_iter.next().is_none());

                return result.into();
            }
        }
    }

    assert!(source_iter.next().is_none());

    result.into()
}

pub struct Costs {
    pub copy: i32,
    pub replace: i32,
    pub delete: i32,
    pub insert: i32,
    pub twiddle: i32,
    pub kill: i32,
}

#[derive(Clone)]
enum InternalOperation {
    None,
    Copy,
    Replace,
    Delete,
    Insert,
    Twiddle,
    Kill,
}

pub fn find_optimal_transform_sequence<T: Eq + Clone>(
    source: &[T],
    target: &[T],
    costs: &Costs,
) -> (Box<[Operation<T>]>, i32) {
    // d(i, j) = edit_distance(source[i..], target[j..]).
    //
    // Optimal cost(i, j) if first operation is:
    //
    //     Copy    => cost(copy)    + d(i + 1, j + 1),
    //     Replace => cost(replace) + d(i + 1, j + 1),
    //     Delete  => cost(delete)  + d(i + 1, j),
    //     Insert  => cost(insert)  + d(i, j + 1),
    //     Twiddle => cost(twiddle) + d(i + 2, j + 2),
    //     Kill    => cost(kill)

    let rows = source.len() + 1;
    let columns = target.len() + 1;
    let mut cache = vec![(InternalOperation::None, 0); columns * rows];

    // Base cases: from empty string to target[j..]. Only insertions can be used.

    for j in (0..target.len()).rev() {
        cache[columns * source.len() + j] = (
            InternalOperation::Insert,
            costs.insert + cache[columns * source.len() + (j + 1)].1,
        );
    }

    // Base cases: from source[i..] to empty string. Only deletion and kill can be used.

    for i in (0..source.len()).rev() {
        let cost_if_delete = costs.delete + cache[columns * (i + 1) + target.len()].1;

        cache[columns * i + target.len()] = if cost_if_delete < costs.kill {
            (InternalOperation::Delete, cost_if_delete)
        } else {
            (InternalOperation::Kill, costs.kill)
        };
    }

    // Inductive cases: from source[i..] to target[j..].

    for (i, source_i) in source.iter().enumerate().rev() {
        for (j, target_j) in target.iter().enumerate().rev() {
            // We put replace first, because this operation is always valid.

            // Replace.

            let mut best_choice = (
                InternalOperation::Replace,
                costs.replace + cache[columns * (i + 1) + (j + 1)].1,
            );

            // Copy.

            if source_i == target_j {
                let cost_if_copy = costs.copy + cache[columns * (i + 1) + (j + 1)].1;

                if cost_if_copy < best_choice.1 {
                    best_choice = (InternalOperation::Copy, cost_if_copy);
                }
            }

            // Delete.

            let cost_if_delete = costs.delete + cache[columns * (i + 1) + j].1;

            if cost_if_delete < best_choice.1 {
                best_choice = (InternalOperation::Delete, cost_if_delete);
            }

            // Insert.

            let cost_if_insert = costs.insert + cache[columns * i + (j + 1)].1;

            if cost_if_insert < best_choice.1 {
                best_choice = (InternalOperation::Insert, cost_if_insert);
            }

            // Twiddle.

            if Some(source_i) == target.get(j + 1) && source.get(i + 1) == Some(target_j) {
                let cost_if_twiddle = costs.twiddle + cache[columns * (i + 2) + (j + 2)].1;

                if cost_if_twiddle < best_choice.1 {
                    best_choice = (InternalOperation::Twiddle, cost_if_twiddle)
                }
            }

            // Kill is not valid here.

            cache[columns * i + j] = best_choice;
        }
    }

    // Construct solution.

    let mut operations = vec![];
    let mut i = 0;
    let mut j = 0;

    loop {
        match cache[columns * i + j].0 {
            InternalOperation::None => break,
            InternalOperation::Copy => {
                operations.push(Operation::Copy);
                i += 1;
                j += 1;
            }
            InternalOperation::Replace => {
                operations.push(Operation::Replace(target[j].clone()));
                i += 1;
                j += 1;
            }
            InternalOperation::Delete => {
                operations.push(Operation::Delete);
                i += 1;
            }
            InternalOperation::Insert => {
                operations.push(Operation::Insert(target[j].clone()));
                j += 1;
            }
            InternalOperation::Twiddle => {
                operations.push(Operation::Twiddle);
                i += 2;
                j += 2;
            }
            InternalOperation::Kill => {
                operations.push(Operation::Kill);
                break;
            }
        }
    }

    (operations.into(), cache[0].1)
}

pub fn align_sequences<T: Eq + Clone>(source: &[T], target: &[T]) -> (Box<[Operation<T>]>, i32) {
    let costs = Costs {
        copy: -1,
        replace: 1,
        delete: 2,
        insert: 2,
        twiddle: i32::MAX,
        kill: i32::MAX,
    };

    let (operations, cost) = find_optimal_transform_sequence(source, target, &costs);

    (operations, -cost)
}

#[cfg(test)]
mod tests {
    use super::{align_sequences, find_optimal_transform_sequence, transform, Costs, Operation};

    type TransformTestCase<'a> = ((&'a [u8], &'a [Operation<u8>]), &'a [u8]);

    const TRANSFORM_TEST_CASES: &[TransformTestCase] = &[
        ((b"", &[]), b""),
        ((b"a", &[Operation::Copy]), b"a"),
        ((b"al", &[Operation::Copy, Operation::Copy]), b"al"),
        (
            (b"alg", &[Operation::Copy, Operation::Copy, Operation::Replace(b't')]),
            b"alt",
        ),
        (
            (
                b"algo",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                ],
            ),
            b"alt",
        ),
        (
            (
                b"algor",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                ],
            ),
            b"altr",
        ),
        (
            (
                b"algor",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                ],
            ),
            b"altru",
        ),
        (
            (
                b"algor",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                    Operation::Insert(b'i'),
                ],
            ),
            b"altrui",
        ),
        (
            (
                b"algor",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                    Operation::Insert(b'i'),
                    Operation::Insert(b's'),
                ],
            ),
            b"altruis",
        ),
        (
            (
                b"algorit",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                    Operation::Insert(b'i'),
                    Operation::Insert(b's'),
                    Operation::Twiddle,
                ],
            ),
            b"altruisti",
        ),
        (
            (
                b"algorit",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                    Operation::Insert(b'i'),
                    Operation::Insert(b's'),
                    Operation::Twiddle,
                    Operation::Insert(b'c'),
                ],
            ),
            b"altruistic",
        ),
        (
            (
                b"algorithm",
                &[
                    Operation::Copy,
                    Operation::Copy,
                    Operation::Replace(b't'),
                    Operation::Delete,
                    Operation::Copy,
                    Operation::Insert(b'u'),
                    Operation::Insert(b'i'),
                    Operation::Insert(b's'),
                    Operation::Twiddle,
                    Operation::Insert(b'c'),
                    Operation::Kill,
                ],
            ),
            b"altruistic",
        ),
    ];

    #[test]
    fn test_transform() {
        for ((source, operations), expected) in TRANSFORM_TEST_CASES.iter().copied() {
            assert_eq!(*transform(source, operations), *expected);
        }
    }

    #[test]
    fn test_find_optimal_transform_sequence() {
        let source = b"algorithm";
        let target = b"altruistic";

        let costs = Costs {
            copy: 1,
            replace: 3,
            delete: 2,
            insert: 2,
            twiddle: 1,
            kill: 1,
        };

        let (operations, cost) = find_optimal_transform_sequence(source, target, &costs);

        let expected_operations = [
            Operation::Copy,
            Operation::Copy,
            Operation::Replace(b't'),
            Operation::Delete,
            Operation::Copy,
            Operation::Insert(b'u'),
            Operation::Insert(b'i'),
            Operation::Insert(b's'),
            Operation::Twiddle,
            Operation::Insert(b'c'),
            Operation::Kill,
        ];

        assert_eq!(*operations, expected_operations);
        assert_eq!(cost, 18);
    }

    #[test]
    fn test_align_sequences() {
        let (operations, score) = align_sequences(b"GATCGGCAT", b"CAATGTGAATC");

        // GATCG GCAT
        // CAATGTGAATC
        // -+--+*+-++*

        let expected_operations = [
            Operation::Replace(b'C'),
            Operation::Copy,
            Operation::Replace(b'A'),
            Operation::Replace(b'T'),
            Operation::Copy,
            Operation::Insert(b'T'),
            Operation::Copy,
            Operation::Replace(b'A'),
            Operation::Copy,
            Operation::Copy,
            Operation::Insert(b'C'),
        ];

        assert_eq!(*operations, expected_operations);
        assert_eq!(score, -3);
    }
}
