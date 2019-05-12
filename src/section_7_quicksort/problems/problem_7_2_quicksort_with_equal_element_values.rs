use std::cmp::Ordering;

pub fn partition_three_way<T: Ord>(a: &mut [T], p: usize, r: usize) -> (usize, usize) {
    let (x, s) = a[..r].split_last_mut().unwrap();

    let mut i = p;
    let mut j = p;
    let mut k = s.len();

    // All elements in s[p..i] < x.
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

    a.swap(k, r - 1);

    (i, k + 1)
}

pub fn quicksort_three_way<T: Ord>(a: &mut [T], m: usize, n: usize) {
    if n - m > 0 {
        let (p, q) = partition_three_way(a, m, n);

        quicksort_three_way(a, m, p);
        quicksort_three_way(a, q, n);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::quicksort_three_way;

    #[test]
    fn test_quicksort_three_way() {
        run_all_sorting_tests(|a| quicksort_three_way(a, 0, a.len()));
    }
}
