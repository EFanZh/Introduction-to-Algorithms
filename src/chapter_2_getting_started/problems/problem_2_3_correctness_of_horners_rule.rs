use std::ops::{Add, Mul};

// 1  y = 0
// 2  for i = n downto 0
// 3      y = a_i + x ⋅ y

#[allow(single_use_lifetimes)] //See <https://github.com/rust-lang/rust/issues/55058>.
pub fn polynomial<T: Default>(a: &[T], x: &T) -> T
where
    for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
{
    let mut y: T = T::default();

    for a_i in a.iter().rev() {
        y = a_i + &(x * &y);
    }

    y
}

#[allow(single_use_lifetimes)] //See <https://github.com/rust-lang/rust/issues/55058>.
pub fn polynomial_naive<T: Default + Clone>(a: &[T], x: &T) -> T
where
    for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
{
    let mut y: T = T::default();

    for (i, a_i) in a.iter().enumerate() {
        let mut p = a_i.clone();

        for _ in 0..i {
            p = &p * x;
        }

        y = &y + &p;
    }

    y
}

#[cfg(test)]
mod tests {
    use super::{polynomial, polynomial_naive};
    use approx::assert_relative_eq;

    fn run_polynomial_test(f: fn(&[f64], &f64) -> f64) {
        assert_relative_eq!(f(&[], &0.0), 0.0);
        assert_relative_eq!(f(&[], &1.0), 0.0);
        assert_relative_eq!(f(&[1.0], &0.0), 1.0);
        assert_relative_eq!(f(&[1.0], &1.0), 1.0);
        assert_relative_eq!(f(&[1.0, 2.0], &0.0), 1.0);
        assert_relative_eq!(f(&[1.0, 2.0], &1.0), 3.0);
        assert_relative_eq!(f(&[1.0, 2.0], &2.0), 5.0);
        assert_relative_eq!(f(&[1.0, 2.0, 3.0], &0.0), 1.0);
        assert_relative_eq!(f(&[1.0, 2.0, 3.0], &1.0), 6.0);
        assert_relative_eq!(f(&[1.0, 2.0, 3.0], &2.0), 17.0);
    }

    #[test]
    fn test_polynomial() {
        run_polynomial_test(polynomial);
    }

    #[test]
    fn test_polynomial_naive() {
        run_polynomial_test(polynomial_naive);
    }
}
