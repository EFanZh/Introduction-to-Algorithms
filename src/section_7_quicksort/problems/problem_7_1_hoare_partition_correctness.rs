// Hoare-Partition(A, p, r)
//  1  x = A[p]
//  2  i = p - 1
//  3  j = r + 1
//  4  while True
//  5      repeat
//  6          j = j - 1
//  7      until A[j] ≤ x
//  8      repeat
//  9          i = i + 1
// 10      until A[i] ≥ x
// 11      if i < j
// 12          exchange A[i] with A[j]
// 13      else return j

pub fn hoare_partition<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) -> usize {
    let x = a[p].clone();
    let mut i = p;
    let mut j = r;

    loop {
        loop {
            j -= 1;

            if a[j] <= x {
                break;
            }
        }

        while a[i] < x {
            i += 1;
        }

        if i < j {
            a.swap(i, j);

            i += 1;
        } else {
            return j + 1;
        }
    }
}

pub fn hoare_quicksort<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q = hoare_partition(a, p, r);

        hoare_quicksort(a, p, q);
        hoare_quicksort(a, q, r);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::hoare_quicksort;

    #[test]
    fn test_hoare_quicksort() {
        run_all_sorting_tests(|a| hoare_quicksort(a, 0, a.len()));
    }
}
