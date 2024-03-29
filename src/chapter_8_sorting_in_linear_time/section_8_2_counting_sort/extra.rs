use std::mem;

pub fn counting_sort_by_key<T: Clone, F: FnMut(&T) -> usize>(a: &mut [T], empty_value: T, mut f: F) {
    let mut b = vec![empty_value; a.len()];
    let mut c = Vec::new();

    for x in a.iter().map(&mut f) {
        if x < c.len() {
            c[x] += 1;
        } else {
            c.resize(x, 0);
            c.push(1);
        }
    }

    // C[i] now contains the number of elements equal to i.

    for i in 1..c.len() {
        c[i] += c[i - 1];
    }

    // C[i] now contains the number of elements less than or equal to i.

    for x in a.iter_mut().rev() {
        let c_key = &mut c[f(x)];

        *c_key -= 1;

        mem::swap(&mut b[*c_key], x);
    }

    // Put elements back to a.

    a.swap_with_slice(&mut b);
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_counting_sort_by_key() {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                test_utilities::assign_vec_from_iter(&mut a, iter::repeat_with(|| rng.gen_range(0..n)).take(n));
                test_utilities::assign_vec(&mut b, &a);

                super::counting_sort_by_key(&mut a, 0, |x| *x);

                b.sort_unstable();

                assert_eq!(a, b);
            }
        }
    }
}
