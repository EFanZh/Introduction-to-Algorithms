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
            min = a_i
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utilities::loop_on_all_unordered_sequences;
    use super::minimum;

    #[test]
    fn test_minimum() {
        loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            if sequence.len() > 0 {
                assert_eq!(minimum(sequence), sorted_sequence.first().unwrap());
            }
        });
    }
}
