pub mod exercises;

// Minimum(A)
//
// 1  min = A[1]
// 2  for i = 2 to A.length
// 3      if min > A[i]
// 4          min = A[i]
// 5  return min

pub fn minimum<T: Ord>(a: &[T]) -> &T {
    let (mut min, a) = a.split_first().unwrap();

    for a_i in a {
        if min > a_i {
            min = a_i;
        }
    }

    min
}

pub fn minimum_and_maximum<T: Ord>(a: &[T]) -> (&T, &T) {
    let mut iterator = a.iter();
    let first = iterator.next().unwrap();

    if let Some(second) = iterator.next() {
        let (mut min, mut max) = if first > second {
            (second, first)
        } else {
            (first, second)
        };

        loop {
            if let Some(first) = iterator.next() {
                if let Some(second) = iterator.next() {
                    let (new_min, new_max) = if first > second {
                        (second, first)
                    } else {
                        (first, second)
                    };

                    if new_min < min {
                        min = new_min;
                    }

                    if new_max > max {
                        max = new_max;
                    }
                } else if first < min {
                    break (first, max);
                } else if first > max {
                    break (min, first);
                } else {
                    break (min, max);
                }
            } else {
                break (min, max);
            }
        }
    } else {
        (first, first)
    }
}

#[cfg(test)]
mod tests {
    use super::{minimum, minimum_and_maximum};
    use crate::test_utilities::loop_on_all_unordered_sequences;

    #[test]
    fn test_minimum() {
        loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            if let Some(expected_minimum) = sorted_sequence.first() {
                assert_eq!(minimum(sequence), expected_minimum);
            }
        });
    }

    #[test]
    fn test_minimum_and_maximum() {
        loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            if let Some(expected_minimum) = sorted_sequence.first() {
                if let Some(expected_maximum) = sorted_sequence.last() {
                    let (minimum, maximum) = minimum_and_maximum(sequence);

                    assert_eq!(minimum, expected_minimum);
                    assert_eq!(maximum, expected_maximum);
                }
            }
        });
    }
}
