mod problem_2_2 {
    use super::super::{
        super::{
            super::section_2_3_designing_algorithms::problems::problem_2_2::bubble_sort,
            utilities::run_all_sorting_tests,
        },
        Bencher,
    };

    #[bench]
    fn test_bubble_sort(b: &mut Bencher) {
        b.iter(|| run_all_sorting_tests(bubble_sort));
    }
}

mod problem_2_3 {
    use super::super::super::super::section_2_3_designing_algorithms::problems::problem_2_3::{
        polynomial, polynomial_naive,
    };

    fn run_polynomial_test(f: fn(&[f64], &f64) -> f64) {
        assert_eq!(f(&[], &0.0), 0.0);
        assert_eq!(f(&[], &1.0), 0.0);
        assert_eq!(f(&[1.0], &0.0), 1.0);
        assert_eq!(f(&[1.0], &1.0), 1.0);
        assert_eq!(f(&[1.0, 2.0], &0.0), 1.0);
        assert_eq!(f(&[1.0, 2.0], &1.0), 3.0);
        assert_eq!(f(&[1.0, 2.0], &2.0), 5.0);
        assert_eq!(f(&[1.0, 2.0, 3.0], &0.0), 1.0);
        assert_eq!(f(&[1.0, 2.0, 3.0], &1.0), 6.0);
        assert_eq!(f(&[1.0, 2.0, 3.0], &2.0), 17.0);
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
