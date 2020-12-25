pub mod exercises;
pub mod extra;

// Counting-Sort(A, B, k)
//
//  1  let C[0‥k] be a new array
//  2  for i = 0 to k
//  3      C[i] = 0
//  4  for j = 1 to A.length
//  5      C[A[j]] = C[A[j]] + 1
//  6  // C[i] now contains the number of elements equal to i.
//  7  for i = 1 to k
//  8      C[i] = C[i] + C[i - 1]
//  9  // C[i] now contains the number of elements less than or equal to i.
// 10  for j = A.length downto 1
// 11      B[C[A[j]]] = A[j]
// 12      C[A[j]] = C[A[j]] - 1

pub fn counting_sort(a: &[usize], b: &mut [usize], k: usize) {
    let mut c = vec![0; k];

    for &x in a {
        c[x] += 1;
    }

    // C[i] now contains the number of elements equal to i.

    for i in 1..k {
        c[i] += c[i - 1];
    }

    // C[i] now contains the number of elements less than or equal to i.

    for &x in a.iter().rev() {
        let c_x = &mut c[x];

        *c_x -= 1;

        b[*c_x] = x;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utilities::assign_vec_from_iter;
    use super::counting_sort;
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_counting_sort() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..2_usize.pow(n as _) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen_range(0..n)).take(n));
                b.resize(n, 0);

                counting_sort(&a, &mut b, n);

                a.sort_unstable();

                assert_eq!(a, b);
            }
        }
    }
}
