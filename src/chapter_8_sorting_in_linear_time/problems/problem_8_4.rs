use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn partition_by_key<'a, T: PartialOrd<K>, K>(a: &'a mut [T], key: &K) -> (&'a mut [T], &'a mut [T], &'a mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = a.len();

    // All elements in a[0..i] < key.
    // All elements in a[i..j] = key.
    // All elements in a[j..k] = unknown.
    // All elements in a[k..] > key.

    while j < k {
        match a[j].partial_cmp(key).unwrap() {
            Ordering::Less => {
                a.swap(j, i);

                i += 1;
                j += 1;
            }
            Ordering::Equal => {
                j += 1;
            }
            Ordering::Greater => {
                k -= 1;

                a.swap(j, k);
            }
        }
    }

    let (s1, s2) = a.split_at_mut(j);
    let (s0, s1) = s1.split_at_mut(i);

    (s0, s1, s2)
}

pub fn group_water_jugs<T, U>(red_water_jugs: &mut [T], blue_water_jugs: &mut [U])
where
    T: PartialOrd<U>,
    U: PartialOrd<T>,
{
    fn helper<T, U, R: Rng>(red_water_jugs: &mut [T], blue_water_jugs: &mut [U], rng: &mut R)
    where
        T: PartialOrd<U>,
        U: PartialOrd<T>,
    {
        if red_water_jugs.len() > 1 {
            let (blue_less, blue_equal, blue_greater) =
                partition_by_key(blue_water_jugs, red_water_jugs.choose(rng).unwrap());

            let (red_less, _, red_greater) = partition_by_key(red_water_jugs, blue_equal.first().unwrap());

            helper(red_less, blue_less, rng);
            helper(red_greater, blue_greater, rng);
        }
    }

    helper(red_water_jugs, blue_water_jugs, &mut thread_rng());
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::{assign_vec, assign_vec_from_iter};
    use super::group_water_jugs;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};
    use std::iter;

    #[test]
    fn test_group_water_jugs() {
        let mut red_water_jugs = Vec::new();
        let mut blue_water_jugs = Vec::new();
        let mut rng = thread_rng();

        for n in 0usize..10 {
            for _ in 0..2usize.pow(n as _) {
                assign_vec_from_iter(&mut red_water_jugs, iter::repeat_with(|| rng.gen_range(0, n)).take(n));
                assign_vec(&mut blue_water_jugs, &red_water_jugs);

                blue_water_jugs.shuffle(&mut rng);

                group_water_jugs(&mut red_water_jugs, &mut blue_water_jugs);

                assert_eq!(red_water_jugs, blue_water_jugs);
            }
        }
    }
}
