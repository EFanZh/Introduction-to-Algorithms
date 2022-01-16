use rand::Rng;
use std::cmp::Ordering;

#[allow(clippy::many_single_char_names)]
pub fn partition_prime<T: Ord>(values: &mut [T], p: usize, r: usize) -> (usize, usize) {
    let (pivot, sub_array) = values[..r].split_last_mut().unwrap();
    let pivot_index = sub_array.len();

    let mut q = p;
    let mut t = q;
    let mut u = pivot_index;

    // All elements in sub_array[p..q] < pivot.
    // All elements in sub_array[q..t] = pivot.
    // All elements in sub_array[t..u] is unknown.
    // All elements in sub_array[u..] > pivot.

    while t < u {
        match sub_array[t].cmp(pivot) {
            Ordering::Less => {
                sub_array.swap(q, t);

                q += 1;
                t += 1;
            }
            Ordering::Equal => {
                t += 1;
            }
            Ordering::Greater => {
                u -= 1;

                sub_array.swap(t, u);
            }
        }
    }

    values.swap(t, pivot_index);

    (q, t + 1)
}

pub fn randomized_partition_prime<T: Ord>(a: &mut [T], p: usize, r: usize) -> (usize, usize) {
    let i = rand::thread_rng().gen_range(p..r);

    a.swap(r - 1, i);

    partition_prime(a, p, r)
}

pub fn quicksort_prime<T: Ord>(values: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let (q, t) = randomized_partition_prime(values, p, r);

        quicksort_prime(values, p, q);
        quicksort_prime(values, t, r);
    }
}

#[cfg(test)]
mod tests {
    use super::quicksort_prime;
    use crate::test_utilities::run_all_sorting_tests;

    #[test]
    fn test_quicksort_prime() {
        run_all_sorting_tests(|a| quicksort_prime(a, 0, a.len()));
    }
}
