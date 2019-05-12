use std::cmp::Ordering;

pub fn partition<T: Ord>(a: &mut [T]) -> usize {
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

    let middle = a.len() / 2;

    middle.max(i).min(k)
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::partition;

    #[test]
    fn test_partition_middle_on_same_elements() {
        fn run_single_test(mut a: Vec<i32>) {
            assert_eq!(partition(&mut a), a.len() / 2);
        }

        run_single_test(vec![0]);
        run_single_test(vec![0, 0]);
        run_single_test(vec![0, 0, 0]);
        run_single_test(vec![0, 0, 0, 0]);
        run_single_test(vec![0, 0, 0, 0, 0]);
        run_single_test(vec![0, 0, 0, 0, 0, 0]);
        run_single_test(vec![0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_partition_by_quicksort() {
        pub fn quicksort<T: Ord>(a: &mut [T]) {
            if a.len() > 1 {
                let q = partition(a);

                quicksort(&mut a[..q]);
                quicksort(&mut a[q + 1..]);
            }
        }

        run_all_sorting_tests(quicksort);
    }
}
