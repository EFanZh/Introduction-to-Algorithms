pub fn preprocess_numbers_in_range(a: &[usize], k: usize) -> Vec<usize> {
    let mut c = vec![0; k];

    for &x in a {
        c[x] += 1;
    }

    // C[i] now contains the number of elements equal to i.

    for i in 1..k {
        c[i] += c[i - 1];
    }

    // C[i] now contains the number of elements less than or equal to i.

    c
}

pub fn count_numbers_in_range(c: &[usize], a: usize, b: usize) -> usize {
    if a <= b && a < c.len() {
        let low = if a == 0 { 0 } else { c[a - 1] };
        let high = if b < c.len() { c[b] } else { *c.last().unwrap() };

        high - low
    } else {
        0
    }
}

pub fn make_range_counter(a: &[usize], k: usize) -> impl Fn(usize, usize) -> usize {
    let c = preprocess_numbers_in_range(a, k);

    move |a, b| count_numbers_in_range(&c, a, b)
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::test_utilities::assign_vec_from_iter;
    use super::make_range_counter;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_make_range_counter() {
        let mut array = Vec::new();
        let mut b = Vec::new();
        let mut rng = thread_rng();

        for n in 0usize..10 {
            for _ in 0..2usize.pow(n as _) {
                assign_vec_from_iter(&mut array, (0..n).map(|_| rng.gen_range(0, n)));
                b.resize(n, 0);

                let counter = make_range_counter(&array, n);

                for a in 0..n + 2 {
                    for b in 0..n + 2 {
                        let our_result = counter(a, b);
                        let expected_result = array.iter().filter(|&&x| x >= a && x <= b).count();

                        assert_eq!(our_result, expected_result);
                    }
                }
            }
        }
    }
}