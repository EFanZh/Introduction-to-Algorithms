use super::super::section_7_1_description_of_quicksort;
use crate::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice;
use crate::utilities::KeyValuePair;
use rand::seq::index;
use rand::Rng;

fn median_of_3<T: Ord>(values: &[T], i: usize, j: usize, k: usize) -> usize {
    let mut t = [
        KeyValuePair::new(&values[i], i),
        KeyValuePair::new(&values[j], j),
        KeyValuePair::new(&values[k], k),
    ];

    insertion_sort_slice(&mut t);

    t[1].value
}

pub fn median_of_3_partition<T: Ord, R: Rng>(a: &mut [T], rng: &mut R) -> usize {
    let (i, j, k) = {
        let indices = index::sample(rng, a.len(), 3);

        (indices.index(0), indices.index(1), indices.index(2))
    };

    let pivot = median_of_3(a, i, j, k);

    a.swap(pivot, a.len() - 1);

    section_7_1_description_of_quicksort::partition(a, 0, a.len())
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

    helper(a, &mut rand::thread_rng());
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_median_of_3_quicksort() {
        test_utilities::run_all_sorting_tests(super::median_of_3_quicksort);
    }
}
