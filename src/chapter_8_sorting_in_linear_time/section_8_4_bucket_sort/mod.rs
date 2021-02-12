use super::super::chapter_2_getting_started::section_2_1_insertion_sort::extra::insertion_sort_slice_by;

pub mod exercises;
pub mod extra;

// Bucket-Sort(A)
//
// 1  n = A.length
// 2  let B[0‥n - 1] be a new array
// 3  for i = 0 to n - 1
// 4      make B[i] an empty list
// 5  for i = 1 to n
// 6      insert A[i] into list B[⌊n A[i]⌋]
// 7  for i = 0 to n - 1
// 8      sort list B[i] with insertion sort
// 9  concatenate the lists B[0], B[1], …, B[n - 1] together in order

pub fn bucket_sort(a: &mut [f64]) {
    let n = a.len();
    let mut b = vec![Vec::new(); n];

    for &a_i in a.iter() {
        #[allow(clippy::cast_precision_loss)]
        b[(n as f64 * a_i) as usize].push(a_i);
    }

    for b_i in &mut b {
        insertion_sort_slice_by(b_i, |lhs, rhs| lhs.partial_cmp(rhs).unwrap());
    }

    for (lhs, rhs) in a.iter_mut().zip(b.into_iter().flatten()) {
        *lhs = rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utilities::{assign_vec, assign_vec_from_iter};
    use super::bucket_sort;
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_bucket_sort() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..2_usize.pow(n as _) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen()).take(n));
                assign_vec(&mut b, &a);

                bucket_sort(&mut b);

                a.sort_unstable_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

                assert_eq!(a, b);
            }
        }
    }
}
