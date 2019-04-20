use crate::utilities::MaxSentinel;

pub mod exercises;
pub mod extra;

// Merge(A, p, q, r)
//
//  1  n1 = q - p + 1
//  2  n2 = r - q
//  3  let L[1‥n1 + 1] and R[1‥n2 + 1] be new arrays
//  4  for i = 1 to n1
//  5      L[i] = A[p + i - 1]
//  6  for j = 1 to n2
//  7      R[j] = A[q + j]
//  8  L[n1 + 1] = ∞
//  9  R[n2 + 1] = ∞
// 10  i = 1
// 11  j = 1
// 12  for k = p to r
// 13      if L[i] ≤ R[j]
// 14          A[k] = L[i]
// 15          i = i + 1
// 16      else A[k] = R[j]
// 17          j = j + 1

pub fn merge<T: Clone + Ord>(a: &mut [T], p: usize, q: usize, r: usize) {
    let n1 = q - p;
    let n2 = r - q;
    let (mut left, mut right) = (vec![Default::default(); n1 + 1], vec![Default::default(); n2 + 1]);

    for i in 0..n1 {
        left[i] = a[p + i].clone().into();
    }

    for j in 0..n2 {
        right[j] = a[q + j].clone().into();
    }

    left[n1] = MaxSentinel::max();
    right[n2] = MaxSentinel::max();

    let mut i = 0;
    let mut j = 0;

    for a_k in &mut a[p..r] {
        if left[i] <= right[j] {
            *a_k = left[i].take_unwrap();
            i += 1;
        } else {
            *a_k = right[j].take_unwrap();
            j += 1;
        }
    }
}

// Merge-Sort(A, p, r)
//
// 1  if p < r
// 2      q = ⌊(p + r) / 2⌋
// 3      Merge-Sort(A, p, q)
// 4      Merge-Sort(A, q + 1, r)
// 5      Merge(A, p, q, r)

pub fn merge_sort<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q = p + (r - p) / 2;

        merge_sort(a, p, q);
        merge_sort(a, q, r);
        merge(a, p, q, r);
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use crate::test_utilities::run_all_sorting_tests;

    fn merge_sort_helper<T: Ord + Clone>(a: &mut [T]) {
        merge_sort(a, 0, a.len());
    }

    #[test]
    fn test_merge_sort() {
        run_all_sorting_tests(merge_sort_helper);
    }

}
