use std::borrow::Borrow;

#[allow(clippy::option_if_let_else)] // False positive.
pub fn sort_strings<T: Borrow<[u8]>>(strs: &mut [T]) {
    fn helper<T: Borrow<[u8]>>(strs: &mut [T], start: usize) {
        if strs.len() > 1 {
            let mut c = [(0, 0); u8::MAX as usize + 1];

            // Count strings with certain letters in position `start`.

            let num_empty_strs = {
                let mut k = 0;

                for i in 0..strs.len() {
                    if let Some(key) = strs[i].borrow().get(start) {
                        c[*key as usize].1 += 1;
                    } else {
                        strs.swap(k, i);

                        k += 1;
                    }
                }

                k
            };

            // Compute bucket positions.

            {
                let (key, num) = &mut c[0];

                *key = num_empty_strs;
                *num += num_empty_strs;
            }

            for i in 1..c.len() {
                let t = c[i - 1].1;
                let c_i = &mut c[i];

                c_i.0 = t;
                c_i.1 += t;
            }

            // Put elements into buckets.

            let mut previous_bucket_end = num_empty_strs;

            for i in 0..c.len() {
                let (mut j, j_end) = c[i];

                while j < j_end {
                    let key = strs[j].borrow()[start] as usize;

                    if key == i {
                        // strs[j] is in its place, skip this.

                        j += 1;
                    } else {
                        // strs[j] is not in its place, swap with the element in strs[j]’s place.

                        let mut c_key = &mut c[key];

                        strs.swap(j, c_key.0);

                        c_key.0 += 1;
                    }
                }

                // Sort strings in the current bucket recursively.

                helper(&mut strs[previous_bucket_end..j_end], start + 1);

                previous_bucket_end = j_end;
            }
        }
    }

    helper(strs, 0);
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_sort_strings() {
        fn random_str<R: Rng>(max_length: usize, rng: &mut R) -> Box<[u8]> {
            let length = rng.gen_range(0..=max_length);

            iter::repeat_with(|| rng.gen()).take(length).collect()
        }

        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for max_length in 0..10 {
                for _ in 0..(1 << n) {
                    test_utilities::assign_vec_from_iter(
                        &mut a,
                        iter::repeat_with(|| random_str(max_length, &mut rng)).take(n),
                    );

                    test_utilities::assign_vec(&mut b, &a);

                    super::sort_strings(&mut a);

                    b.sort_unstable();

                    assert_eq!(a, b);
                }
            }
        }
    }
}
