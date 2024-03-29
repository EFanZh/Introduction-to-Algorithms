pub mod exercises;

// Radix-Sort(A, d)
// 1  for i = 1 to d
// 2      use a stable sort to sort array A on digit i

pub fn radix_sort<T, S: FnMut(&mut [T], usize)>(a: &mut [T], d: usize, mut stable_sorter: S) {
    for i in (0..d).rev() {
        stable_sorter(a, i);
    }
}

#[cfg(test)]
mod tests {
    use super::super::section_8_2_counting_sort::extra;
    use crate::test_utilities;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_radix_sort() {
        let mut array_1 = Vec::new();
        let mut array_2 = Vec::new();
        let mut rng = rand::thread_rng();

        for n in 0_usize..10 {
            for k in 1..4 {
                for _ in 0..(1 << n) {
                    test_utilities::assign_vec_from_iter(
                        &mut array_1,
                        iter::repeat_with(|| iter::repeat_with(|| rng.gen_range(0..n)).take(k).collect::<Box<_>>())
                            .take(n),
                    );

                    test_utilities::assign_vec(&mut array_2, &array_1);

                    super::radix_sort(&mut array_1, k, |a, i| {
                        extra::counting_sort_by_key(a, Box::new([]), |x| x[i]);
                    });

                    array_2.sort_unstable();

                    assert_eq!(array_1, array_2);
                }
            }
        }
    }
}
