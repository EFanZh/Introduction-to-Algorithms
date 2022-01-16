pub fn counting_sort_by_key_inplace_unstable<T, F: FnMut(&T) -> usize>(values: &mut [T], k: usize, mut f: F) {
    let mut c = vec![(0, 0); k];

    for x in &*values {
        c[f(x)].1 += 1;
    }

    for i in 1..k {
        let t = c[i - 1].1;
        let c_i = &mut c[i];

        c_i.0 = t;
        c_i.1 += t;
    }

    for i in 0..k {
        let (mut j, j_end) = c[i];

        while j < j_end {
            let key = f(&values[j]);

            if key == i {
                // a[j] is in its place, skip this.

                j += 1;
            } else {
                // a[j] is not in its place, swap with the element in a[j]’s place.

                let mut c_key = &mut c[key];

                values.swap(j, c_key.0);

                c_key.0 += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::counting_sort_by_key_inplace_unstable;
    use crate::test_utilities::{assign_vec, assign_vec_from_iter};
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_counting_sort_by_key_inplace_unstable() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen_range(0..n)).take(n));
                assign_vec(&mut b, &a);

                counting_sort_by_key_inplace_unstable(&mut a, n, |x| *x);

                b.sort_unstable();

                assert_eq!(a, b);
            }
        }
    }
}
