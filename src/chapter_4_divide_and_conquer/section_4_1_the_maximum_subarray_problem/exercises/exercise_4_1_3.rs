use num::Zero;

// Find-Maximum-Subarray-Brute-Force(A)
//
//  1  max-left = 0
//  2  max-right = 0
//  3  max-sum = -∞
//  4  for i = 1 to A.Length
//  5      sum = 0
//  6      for j = i to n
//  7          sum = sum + A[j]
//  8          if sum > max-sum
//  9              max-left = i
// 10              max-right = j
// 11              max-sum = sum
// 12  return (max-left, max-right, max-sum)

pub fn find_maximum_subarray_brute_force<T: Clone + Ord + Zero>(a: &[T]) -> (usize, usize, T) {
    let mut max_left = 0_usize;
    let mut max_right = 0_usize;
    let mut max_sum = a[0].clone();

    for i in 0..a.len() {
        let mut sum = T::zero();

        for (j, a_j) in a.iter().enumerate().skip(i) {
            sum = sum + a_j.clone();

            if sum > max_sum {
                max_left = i;
                max_right = j;
                max_sum = sum.clone();
            }
        }
    }

    (max_left, max_right + 1, max_sum)
}

#[cfg(test)]
mod tests {
    use super::super::super::tests;

    #[test]
    fn test_find_maximum_subarray_brute_force() {
        tests::run_find_maximum_subarray_test_cases(super::find_maximum_subarray_brute_force);
    }
}
