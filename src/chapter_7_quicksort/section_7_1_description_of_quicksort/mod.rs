pub mod exercises;
pub mod extra;

// Partition(A, p, r)
//
// 1  x = A[r]
// 2  i = p - 1
// 3  for j = p to r - 1
// 4      if A[j] ≤ x
// 5          i = i + 1
// 6          exchange A[i] with A[j]
// 7  exchange A[i + 1] with A[r]
// 8  return i + 1

#[allow(clippy::many_single_char_names)] // Expected.
pub fn partition<T: Ord>(a: &mut [T], p: usize, r: usize) -> usize {
    let (s, x) = {
        let (slice_1, slice_2) = a.split_at_mut(r - 1);

        (slice_1, &slice_2[0])
    };

    let mut i = p;

    for j in p..r - 1 {
        if s[j] <= *x {
            s.swap(i, j);

            i += 1;
        }
    }

    a.swap(i, r - 1);

    i
}

// Quicksort(A, p, r)
//
// 1  if p < r
// 2      q = Partition(A, p, r)
// 3      Quicksort(A, p, q - 1)
// 4      Quicksort(A, q + 1, r)

pub fn quicksort<T: Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q = partition(a, p, r);

        quicksort(a, p, q);
        quicksort(a, q + 1, r);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_quicksort() {
        test_utilities::run_all_sorting_tests(|a| super::quicksort(a, 0, a.len()));
    }
}
