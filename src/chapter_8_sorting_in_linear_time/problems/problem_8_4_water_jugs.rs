use crate::chapter_7_quicksort::section_7_1_description_of_quicksort::extra;
use rand::seq::SliceRandom;
use rand::Rng;

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
                extra::partition_by_key(blue_water_jugs, red_water_jugs.choose(rng).unwrap());

            let (red_less, _, red_greater) = extra::partition_by_key(red_water_jugs, blue_equal.first().unwrap());

            helper(red_less, blue_less, rng);
            helper(red_greater, blue_greater, rng);
        }
    }

    helper(red_water_jugs, blue_water_jugs, &mut rand::thread_rng());
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_group_water_jugs() {
        let mut red_water_jugs = Vec::new();
        let mut blue_water_jugs = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for _ in 0..(1 << n) {
                test_utilities::assign_vec_from_iter(
                    &mut red_water_jugs,
                    iter::repeat_with(|| rng.gen_range(0..n)).take(n),
                );

                test_utilities::assign_vec(&mut blue_water_jugs, &red_water_jugs);

                blue_water_jugs.shuffle(&mut rng);

                super::group_water_jugs(&mut red_water_jugs, &mut blue_water_jugs);

                assert_eq!(red_water_jugs, blue_water_jugs);
            }
        }
    }
}
