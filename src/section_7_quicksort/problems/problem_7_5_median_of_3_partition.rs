use super::super::super::section_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice;
use super::super::super::utilities::KeyValuePair;
use super::super::section_7_1_description_of_quicksort::partition;
use rand::seq::index;
use rand::{thread_rng, Rng};

fn median_of_3<T: Ord>(a: &[T], i: usize, j: usize, k: usize) -> usize {
    let mut t = [
        KeyValuePair::new(&a[i], i),
        KeyValuePair::new(&a[j], j),
        KeyValuePair::new(&a[k], k),
    ];

    insertion_sort_slice(&mut t);

    t[1].value
}

pub fn median_of_3_partition<T: Ord, R: Rng>(a: &mut [T], rng: &mut R) -> usize {
    let (i, j, k) = {
        let mut iter = index::sample(rng, a.len(), 3).into_iter();

        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    };

    let pivot = median_of_3(a, i, j, k);

    a.swap(pivot, a.len() - 1);

    partition(a, 0, a.len())
}

pub fn median_of_3_quicksort<T: Ord>(a: &mut [T]) {
    fn helper<T: Ord, R: Rng>(a: &mut [T], rng: &mut R) {
        if a.len() > 2 {
            let q = median_of_3_partition(a, rng);

            helper(&mut a[..q], rng);
            helper(&mut a[q + 1..], rng);
        } else {
            insertion_sort_slice(a);
        }
    }

    helper(a, &mut thread_rng());
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::median_of_3_quicksort;

    #[test]
    fn test_median_of_3_quicksort() {
        run_all_sorting_tests(median_of_3_quicksort);
    }
}
