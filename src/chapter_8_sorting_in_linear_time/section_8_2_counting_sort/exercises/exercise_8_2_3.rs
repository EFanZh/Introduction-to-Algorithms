pub fn modified_counting_sort(a: &[usize], b: &mut [usize], k: usize) {
    let mut c = vec![0; k];

    for &x in a {
        c[x] += 1;
    }

    // C[i] now contains the number of elements equal to i.

    for i in 1..k {
        c[i] += c[i - 1];
    }

    // C[i] now contains the number of elements less than or equal to i.

    for &x in a {
        b[c[x] - 1] = x;

        c[x] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::modified_counting_sort;
    use crate::test_utilities::assign_vec_from_iter;
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_modified_counting_sort() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen_range(0..n)).take(n));
                b.resize(n, 0);

                modified_counting_sort(&a, &mut b, n);

                a.sort_unstable();

                assert_eq!(a, b);
            }
        }
    }
}
