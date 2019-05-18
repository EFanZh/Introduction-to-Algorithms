pub fn modified_counting_sort(a: &[usize], b: &mut [usize], k: usize) {
    let mut c = vec![0; k];

    for x in a {
        c[*x] += 1;
    }

    // C[i] now contains the number of elements equal to i.

    for i in 1..k {
        c[i] += c[i - 1];
    }

    // C[i] now contains the number of elements less than or equal to i.

    for x in a {
        b[c[*x] - 1] = *x;

        c[*x] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::assign_vec_from_iter;
    use super::modified_counting_sort;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_modified_counting_sort() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0usize..10 {
            for _ in 0..2usize.pow(n as _) {
                assign_vec_from_iter(&mut a, (0..n).map(|_| rng.gen_range(0, n)));
                b.resize(n, 0);

                modified_counting_sort(&a, &mut b, n);

                a.sort_unstable();

                assert_eq!(a, b);
            }
        }
    }
}
