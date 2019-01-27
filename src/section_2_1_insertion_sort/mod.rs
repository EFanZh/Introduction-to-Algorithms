pub mod exercises;
pub mod extra;

// Insertion-Sort(A)
//
// 1  for j = 2 to A.length
// 2      key = A[j]
// 3      // Insert A[j] into the sorted sequence A[1..j - 1].
// 4      i = j - 1
// 5      while i > 0 and A[i] > key
// 6          A[i + 1] = A[i]
// 7          i = i - 1
// 8      A[i + 1] = key

pub fn insertion_sort<T: Ord + Clone>(a: &mut [T]) {
    for j in 1..a.len() {
        let key = a[j].clone();

        // Insert `a[j]` into the sorted sequence `a[0..j]`.

        let mut i = j - 1;

        while i < a.len() && a[i] > key {
            a[i + 1] = a[i].clone();
            i = i.wrapping_sub(1);
        }

        a[i.wrapping_add(1)] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::insertion_sort;
    use crate::test_utilities::run_all_sorting_tests;
    use test::Bencher;

    #[bench]
    fn test_insertion_sort(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(insertion_sort));
    }
}
