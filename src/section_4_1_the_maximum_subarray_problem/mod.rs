use num_traits::{Bounded, Zero};
use std::ops::Add;

pub mod exercises;
pub mod utilities;

// Find-Max-Crossing-Subarray(A, low, mid, high)
//
//  1  left-sum = -∞
//  2  sum = 0
//  3  for i = mid downto low
//  4      sum = sum + A[i]
//  5      if sum > left-sum
//  6          left-sum = sum
//  7          max-left = i
//  8  right-sum = -∞
//  9  sum = 0
// 10  for j = mid + 1 to high
// 11      sum = sum + A[j]
// 12      if sum > right-sum
// 13          right-sum = sum
// 14          max-right = j
// 15  return (max-left, max-right, left-sum + right-sum)

pub fn find_max_crossing_subarray<T: Add + Bounded + Clone + Ord + Zero>(
    a: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, T) {
    let mut left_sum = a[mid - 1].clone();
    let mut sum = left_sum.clone();
    let mut max_left = mid - 1;

    for i in (low..mid - 1).rev() {
        sum = sum + a[i].clone();

        if sum > left_sum {
            left_sum = sum.clone();
            max_left = i;
        }
    }

    let mut right_sum = a[mid].clone();
    let mut sum = right_sum.clone();
    let mut max_right = mid;

    for (j, a_j) in a.iter().enumerate().take(high).skip(mid + 1) {
        sum = sum + a_j.clone();

        if sum > right_sum {
            right_sum = sum.clone();
            max_right = j;
        }
    }

    (max_left, max_right + 1, left_sum + right_sum)
}

// Find-Maximum-Subarray(A, low, high)
//
//  1  if high == low
//  2      return (low, high, A[low]) // base case: only one element
//  3  else
//  4      mid = ⌊(low + high) / 2⌋
//  5      (left-low, left-high, left-sum) = Find-Maximum-Subarray(A, low, mid)
//  6      (right-low, right-high, right-sum) = Find-Maximum-Subarray(A, mid + 1, high)
//  7      (cross-low, cross-high, cross-sum) = Find-Max-Crossing-Subarray(A, low, mid, high)
//  8      if left-sum ≥ right-sum and left-sum ≥ cross-sum
//  9          return (left-low, left-high, left-sum)
// 10      elseif right-sum ≥ left-sum and right-sum ≥ cross-sum
// 11          return (right-low, right-high, right-sum)
// 12      else
// 13          return (cross-low, cross-high, cross-sum)

pub fn find_maximum_subarray<T: Add + Bounded + Clone + Ord + Zero>(
    a: &[T],
    low: usize,
    high: usize,
) -> (usize, usize, T) {
    if high == low + 1 {
        (low, high, a[low].clone())
    } else {
        let mid = low + (high - low) / 2;
        let (left_low, left_high, left_sum) = find_maximum_subarray(a, low, mid);
        let (right_low, right_high, right_sum) = find_maximum_subarray(a, mid, high);
        let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(a, low, mid, high);

        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_low, left_high, left_sum)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_low, right_high, right_sum)
        } else {
            (cross_low, cross_high, cross_sum)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_maximum_subarray;
    use super::utilities::run_find_maximum_subarray_tests;

    #[test]
    fn test_find_maximum_subarray() {
        run_find_maximum_subarray_tests(|a| find_maximum_subarray(a, 0, a.len()));
    }
}
