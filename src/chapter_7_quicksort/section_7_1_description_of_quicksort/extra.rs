use std::cmp::Ordering;

pub fn partition_by_key<'a, T: PartialOrd<K>, K>(a: &'a mut [T], key: &K) -> (&'a mut [T], &'a mut [T], &'a mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = a.len();

    // All elements in a[0..i] < key.
    // All elements in a[i..j] = key.
    // All elements in a[j..k] = unknown.
    // All elements in a[k..] > key.

    while j < k {
        match a[j].partial_cmp(key).unwrap() {
            Ordering::Less => {
                a.swap(j, i);

                i += 1;
                j += 1;
            }
            Ordering::Equal => {
                j += 1;
            }
            Ordering::Greater => {
                k -= 1;

                a.swap(j, k);
            }
        }
    }

    let (s1, s2) = a.split_at_mut(j);
    let (s0, s1) = s1.split_at_mut(i);

    (s0, s1, s2)
}

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
    if a.len() > 1 {
        let q = partition_slice(a);

        quicksort_slice(&mut a[..q]);
        quicksort_slice(&mut a[q + 1..]);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_quicksort_slice() {
        test_utilities::run_all_sorting_tests(super::quicksort_slice);
    }
}
