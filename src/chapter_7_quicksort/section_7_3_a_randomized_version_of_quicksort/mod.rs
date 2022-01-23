use super::section_7_1_description_of_quicksort;
use rand::Rng;

// Randomized-Partition(A, p, r)
//
// 1  i = Random(p, r)
// 2  exchange A[r] with A[i]
// 3  return Partition(A, p, r)

pub fn randomized_partition<T: Ord>(a: &mut [T], p: usize, r: usize) -> usize {
    let i = rand::thread_rng().gen_range(p..r);

    a.swap(r - 1, i);

    section_7_1_description_of_quicksort::partition(a, p, r)
}

// Randomized-Quicksort(A, p, r)
//
// 1  if p < r
// 2      q = Randomized-Partition(A, p, r)
// 3      Randomized-Quicksort(A, p, q - 1)
// 4      Randomized-Quicksort(A, q + 1, r)

pub fn randomized_quicksort<T: Ord>(a: &mut [T], p: usize, r: usize) {
    if r - p > 1 {
        let q = randomized_partition(a, p, r);

        randomized_quicksort(a, p, q);
        randomized_quicksort(a, q + 1, r);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_randomized_quicksort() {
        test_utilities::run_all_sorting_tests(|a| super::randomized_quicksort(a, 0, a.len()));
    }
}
