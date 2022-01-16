// Bubblesort(A)
//
// 1  for i = 1 to A.length - 1
// 2      for j = A.length downto i + 1
// 3          if A[j] < A[j - 1]
// 4              exchange A[j] with A[j - 1]

pub fn bubble_sort<T: Ord + Clone>(a: &mut [T]) {
    for i in 1..a.len() {
        for j in (i..a.len()).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use crate::test_utilities::run_all_sorting_tests;

    #[test]
    fn test_bubble_sort() {
        run_all_sorting_tests(bubble_sort);
    }
}
