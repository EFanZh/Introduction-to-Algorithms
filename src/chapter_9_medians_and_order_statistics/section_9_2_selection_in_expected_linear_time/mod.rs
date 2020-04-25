use super::super::chapter_7_quicksort::section_7_3_a_randomized_version_of_quicksort::randomized_partition;
use std::cmp::Ordering;

pub mod exercises;

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

pub fn randomized_select<T: Ord>(a: &mut [T], p: usize, r: usize, i: usize) -> &mut T {
    if r - p == 1 {
        return &mut a[p];
    }

    let q = randomized_partition(a, p, r);
    let k = q - p;

    match i.cmp(&k) {
        Ordering::Less => randomized_select(a, p, q, i),
        Ordering::Equal => &mut a[q],
        Ordering::Greater => randomized_select(a, q + 1, r, i - k - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_all_select_test_cases;
    use super::randomized_select;

    #[test]
    fn test_randomized_select() {
        run_all_select_test_cases(randomized_select);
    }
}
