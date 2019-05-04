use std::cmp::Ordering;

pub fn partition_slice<T: Ord>(a: &mut [T]) -> usize {
    let (x, s) = a.split_last_mut().unwrap();

    let mut i = 0;

    for j in 0..s.len() {
        if s[j] < *x {
            s.swap(i, j);

            i += 1;
        }
    }

    a.swap(i, a.len() - 1);

    i
}

pub fn quicksort_slice<T: Ord>(a: &mut [T]) {
    if !a.is_empty() {
        let q = partition_slice(a);

        quicksort_slice(&mut a[..q]);
        quicksort_slice(&mut a[q + 1..]);
    }
}

pub fn partition_three_way<T: Ord>(a: &mut [T]) -> (usize, usize) {
    let (x, s) = a.split_last_mut().unwrap();

    let mut i = 0;
    let mut j = 0;
    let mut k = s.len();

    // All elements in s[0..i] < x.
    // All elements in s[i..j] = x.
    // All elements in s[j..k] = unknown.
    // All elements in s[k..] > x.

    while j < k {
        match s[j].cmp(x) {
            Ordering::Less => {
                s.swap(j, i);

                i += 1;
                j += 1;
            }
            Ordering::Equal => {
                j += 1;
            }
            Ordering::Greater => {
                k -= 1;

                s.swap(j, k);
            }
        }
    }

    a.swap(k, a.len() - 1);

    (i, k + 1)
}

pub fn quicksort_three_way<T: Ord>(a: &mut [T]) {
    if !a.is_empty() {
        let (p, q) = partition_three_way(a);

        quicksort_slice(&mut a[..p]);
        quicksort_slice(&mut a[q..]);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::{quicksort_slice, quicksort_three_way};

    #[test]
    fn test_quicksort_slice() {
        run_all_sorting_tests(quicksort_slice);
    }

    #[test]
    fn test_quicksort_three_way() {
        run_all_sorting_tests(quicksort_three_way);
    }
}
