use super::super::chapter_7_quicksort::section_7_3_a_randomized_version_of_quicksort::randomized_partition;

// Randomized-Select(A, p, r, i)
//
// 1  if p == r
// 2      return A[p]
// 3  q = Randomized-Partition(A, p, r)
// 4  k = q - p + 1
// 5  if i == k // the pivot value is the answer
// 6      return A[q]
// 7  elseif i < k
// 8      return Randomized-Select(A, p, q - 1, i)
// 9  else return Randomized-Select(A, q + 1, r, i - k)

pub fn randomized_select<T: Ord>(a: &mut [T], p: usize, r: usize, i: usize) -> &T {
    if r - p == 1 {
        return &a[p];
    }

    let q = randomized_partition(a, p, r);
    let k = q - p;

    if i == k {
        &a[q]
    } else if i < k {
        randomized_select(a, p, q, i)
    } else {
        randomized_select(a, q + 1, r, i - k - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utilities::{assign_vec, loop_on_all_unordered_sequences};
    use super::randomized_select;

    #[test]
    fn test_randomized_select() {
        let mut buffer = Vec::new();

        loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            for (i, expected_value) in sorted_sequence.iter().enumerate() {
                assign_vec(&mut buffer, sequence);

                assert_eq!(randomized_select(&mut buffer, 0, sequence.len(), i), expected_value);
            }
        })
    }
}
