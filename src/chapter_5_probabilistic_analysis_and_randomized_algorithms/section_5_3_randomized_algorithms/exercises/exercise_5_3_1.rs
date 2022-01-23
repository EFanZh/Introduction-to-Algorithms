use rand::Rng;

pub fn randomize_in_place<T>(a: &mut [T]) {
    if a.len() > 1 {
        let n = a.len();
        let mut rng = rand::thread_rng();

        a.swap(0, rng.gen_range(0..n));

        for i in 1..n {
            a.swap(i, rng.gen_range(i..n));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_randomize_in_place() {
        super::super::super::tests::run_test_randomize(super::randomize_in_place);
    }
}
