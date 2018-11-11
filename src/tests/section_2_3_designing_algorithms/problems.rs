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

mod problem_2_4 {
    use super::super::super::super::section_2_3_designing_algorithms::problems::problem_2_4::inversions;

    #[test]
    fn test_inversions() {
        assert_eq!(inversions(&[0; 0]), 0);

        assert_eq!(inversions(&[0]), 0);

        assert_eq!(inversions(&[0, 0]), 0);
        assert_eq!(inversions(&[0, 1]), 0);
        assert_eq!(inversions(&[1, 0]), 1);

        assert_eq!(inversions(&[0, 0, 0]), 0);
        assert_eq!(inversions(&[0, 0, 1]), 0);
        assert_eq!(inversions(&[0, 1, 0]), 1);
        assert_eq!(inversions(&[0, 1, 1]), 0);
        assert_eq!(inversions(&[0, 1, 2]), 0);
        assert_eq!(inversions(&[0, 2, 1]), 1);
        assert_eq!(inversions(&[1, 0, 0]), 2);
        assert_eq!(inversions(&[1, 0, 1]), 1);
        assert_eq!(inversions(&[1, 0, 2]), 1);
        assert_eq!(inversions(&[1, 1, 0]), 2);
        assert_eq!(inversions(&[1, 2, 0]), 2);
        assert_eq!(inversions(&[2, 0, 1]), 2);
        assert_eq!(inversions(&[2, 1, 0]), 3);

        assert_eq!(inversions(&[0, 0, 0, 0]), 0);
        assert_eq!(inversions(&[0, 0, 0, 1]), 0);
        assert_eq!(inversions(&[0, 0, 1, 0]), 1);
        assert_eq!(inversions(&[0, 0, 1, 1]), 0);
        assert_eq!(inversions(&[0, 0, 1, 2]), 0);
        assert_eq!(inversions(&[0, 0, 2, 1]), 1);
        assert_eq!(inversions(&[0, 1, 0, 0]), 2);
        assert_eq!(inversions(&[0, 1, 0, 1]), 1);
        assert_eq!(inversions(&[0, 1, 0, 2]), 1);
        assert_eq!(inversions(&[0, 1, 1, 0]), 2);
        assert_eq!(inversions(&[0, 1, 1, 1]), 0);
        assert_eq!(inversions(&[0, 1, 1, 2]), 0);
        assert_eq!(inversions(&[0, 1, 2, 0]), 2);
        assert_eq!(inversions(&[0, 1, 2, 1]), 1);
        assert_eq!(inversions(&[0, 1, 2, 2]), 0);
        assert_eq!(inversions(&[0, 1, 2, 3]), 0);
        assert_eq!(inversions(&[0, 1, 3, 2]), 1);
        assert_eq!(inversions(&[0, 2, 0, 1]), 2);
        assert_eq!(inversions(&[0, 2, 1, 0]), 3);
        assert_eq!(inversions(&[0, 2, 1, 1]), 2);
        assert_eq!(inversions(&[0, 2, 1, 2]), 1);
        assert_eq!(inversions(&[0, 2, 1, 3]), 1);
        assert_eq!(inversions(&[0, 2, 2, 1]), 2);
        assert_eq!(inversions(&[0, 2, 3, 1]), 2);
        assert_eq!(inversions(&[0, 3, 1, 2]), 2);
        assert_eq!(inversions(&[0, 3, 2, 1]), 3);
        assert_eq!(inversions(&[1, 0, 0, 0]), 3);
        assert_eq!(inversions(&[1, 0, 0, 1]), 2);
        assert_eq!(inversions(&[1, 0, 0, 2]), 2);
        assert_eq!(inversions(&[1, 0, 1, 0]), 3);
        assert_eq!(inversions(&[1, 0, 1, 1]), 1);
        assert_eq!(inversions(&[1, 0, 1, 2]), 1);
        assert_eq!(inversions(&[1, 0, 2, 0]), 3);
        assert_eq!(inversions(&[1, 0, 2, 1]), 2);
        assert_eq!(inversions(&[1, 0, 2, 2]), 1);
        assert_eq!(inversions(&[1, 0, 2, 3]), 1);
        assert_eq!(inversions(&[1, 0, 3, 2]), 2);
        assert_eq!(inversions(&[1, 1, 0, 0]), 4);
        assert_eq!(inversions(&[1, 1, 0, 1]), 2);
        assert_eq!(inversions(&[1, 1, 0, 2]), 2);
        assert_eq!(inversions(&[1, 1, 1, 0]), 3);
        assert_eq!(inversions(&[1, 1, 2, 0]), 3);
        assert_eq!(inversions(&[1, 2, 0, 0]), 4);
        assert_eq!(inversions(&[1, 2, 0, 1]), 3);
        assert_eq!(inversions(&[1, 2, 0, 2]), 2);
        assert_eq!(inversions(&[1, 2, 0, 3]), 2);
        assert_eq!(inversions(&[1, 2, 1, 0]), 4);
        assert_eq!(inversions(&[1, 2, 2, 0]), 3);
        assert_eq!(inversions(&[1, 2, 3, 0]), 3);
        assert_eq!(inversions(&[1, 3, 0, 2]), 3);
        assert_eq!(inversions(&[1, 3, 2, 0]), 4);
        assert_eq!(inversions(&[2, 0, 0, 1]), 3);
        assert_eq!(inversions(&[2, 0, 1, 0]), 4);
        assert_eq!(inversions(&[2, 0, 1, 1]), 3);
        assert_eq!(inversions(&[2, 0, 1, 2]), 2);
        assert_eq!(inversions(&[2, 0, 1, 3]), 2);
        assert_eq!(inversions(&[2, 0, 2, 1]), 3);
        assert_eq!(inversions(&[2, 0, 3, 1]), 3);
        assert_eq!(inversions(&[2, 1, 0, 0]), 5);
        assert_eq!(inversions(&[2, 1, 0, 1]), 4);
        assert_eq!(inversions(&[2, 1, 0, 2]), 3);
        assert_eq!(inversions(&[2, 1, 0, 3]), 3);
        assert_eq!(inversions(&[2, 1, 1, 0]), 5);
        assert_eq!(inversions(&[2, 1, 2, 0]), 4);
        assert_eq!(inversions(&[2, 1, 3, 0]), 4);
        assert_eq!(inversions(&[2, 2, 0, 1]), 4);
        assert_eq!(inversions(&[2, 2, 1, 0]), 5);
        assert_eq!(inversions(&[2, 3, 0, 1]), 4);
        assert_eq!(inversions(&[2, 3, 1, 0]), 5);
        assert_eq!(inversions(&[3, 0, 1, 2]), 3);
        assert_eq!(inversions(&[3, 0, 2, 1]), 4);
        assert_eq!(inversions(&[3, 1, 0, 2]), 4);
        assert_eq!(inversions(&[3, 1, 2, 0]), 5);
        assert_eq!(inversions(&[3, 2, 0, 1]), 5);
        assert_eq!(inversions(&[3, 2, 1, 0]), 6);

        assert_eq!(inversions(&[0, 0, 0, 0, 0]), 0);
        assert_eq!(inversions(&[0, 0, 0, 0, 1]), 0);
        assert_eq!(inversions(&[0, 0, 0, 1, 0]), 1);
        assert_eq!(inversions(&[0, 0, 0, 1, 1]), 0);
        assert_eq!(inversions(&[0, 0, 0, 1, 2]), 0);
        assert_eq!(inversions(&[0, 0, 0, 2, 1]), 1);
        assert_eq!(inversions(&[0, 0, 1, 0, 0]), 2);
        assert_eq!(inversions(&[0, 0, 1, 0, 1]), 1);
        assert_eq!(inversions(&[0, 0, 1, 0, 2]), 1);
        assert_eq!(inversions(&[0, 0, 1, 1, 0]), 2);
        assert_eq!(inversions(&[0, 0, 1, 1, 1]), 0);
        assert_eq!(inversions(&[0, 0, 1, 1, 2]), 0);
        assert_eq!(inversions(&[0, 0, 1, 2, 0]), 2);
        assert_eq!(inversions(&[0, 0, 1, 2, 1]), 1);
        assert_eq!(inversions(&[0, 0, 1, 2, 2]), 0);
        assert_eq!(inversions(&[0, 0, 1, 2, 3]), 0);
        assert_eq!(inversions(&[0, 0, 1, 3, 2]), 1);
        assert_eq!(inversions(&[0, 0, 2, 0, 1]), 2);
        assert_eq!(inversions(&[0, 0, 2, 1, 0]), 3);
        assert_eq!(inversions(&[0, 0, 2, 1, 1]), 2);
        assert_eq!(inversions(&[0, 0, 2, 1, 2]), 1);
        assert_eq!(inversions(&[0, 0, 2, 1, 3]), 1);
        assert_eq!(inversions(&[0, 0, 2, 2, 1]), 2);
        assert_eq!(inversions(&[0, 0, 2, 3, 1]), 2);
        assert_eq!(inversions(&[0, 0, 3, 1, 2]), 2);
        assert_eq!(inversions(&[0, 0, 3, 2, 1]), 3);
        assert_eq!(inversions(&[0, 1, 0, 0, 0]), 3);
        assert_eq!(inversions(&[0, 1, 0, 0, 1]), 2);
        assert_eq!(inversions(&[0, 1, 0, 0, 2]), 2);
        assert_eq!(inversions(&[0, 1, 0, 1, 0]), 3);
        assert_eq!(inversions(&[0, 1, 0, 1, 1]), 1);
        assert_eq!(inversions(&[0, 1, 0, 1, 2]), 1);
        assert_eq!(inversions(&[0, 1, 0, 2, 0]), 3);
        assert_eq!(inversions(&[0, 1, 0, 2, 1]), 2);
        assert_eq!(inversions(&[0, 1, 0, 2, 2]), 1);
        assert_eq!(inversions(&[0, 1, 0, 2, 3]), 1);
        assert_eq!(inversions(&[0, 1, 0, 3, 2]), 2);
        assert_eq!(inversions(&[0, 1, 1, 0, 0]), 4);
        assert_eq!(inversions(&[0, 1, 1, 0, 1]), 2);
        assert_eq!(inversions(&[0, 1, 1, 0, 2]), 2);
        assert_eq!(inversions(&[0, 1, 1, 1, 0]), 3);
        assert_eq!(inversions(&[0, 1, 1, 1, 1]), 0);
        assert_eq!(inversions(&[0, 1, 1, 1, 2]), 0);
        assert_eq!(inversions(&[0, 1, 1, 2, 0]), 3);
        assert_eq!(inversions(&[0, 1, 1, 2, 1]), 1);
        assert_eq!(inversions(&[0, 1, 1, 2, 2]), 0);
        assert_eq!(inversions(&[0, 1, 1, 2, 3]), 0);
        assert_eq!(inversions(&[0, 1, 1, 3, 2]), 1);
        assert_eq!(inversions(&[0, 1, 2, 0, 0]), 4);
        assert_eq!(inversions(&[0, 1, 2, 0, 1]), 3);
        assert_eq!(inversions(&[0, 1, 2, 0, 2]), 2);
        assert_eq!(inversions(&[0, 1, 2, 0, 3]), 2);
        assert_eq!(inversions(&[0, 1, 2, 1, 0]), 4);
        assert_eq!(inversions(&[0, 1, 2, 1, 1]), 2);
        assert_eq!(inversions(&[0, 1, 2, 1, 2]), 1);
        assert_eq!(inversions(&[0, 1, 2, 1, 3]), 1);
        assert_eq!(inversions(&[0, 1, 2, 2, 0]), 3);
        assert_eq!(inversions(&[0, 1, 2, 2, 1]), 2);
        assert_eq!(inversions(&[0, 1, 2, 2, 2]), 0);
        assert_eq!(inversions(&[0, 1, 2, 2, 3]), 0);
        assert_eq!(inversions(&[0, 1, 2, 3, 0]), 3);
        assert_eq!(inversions(&[0, 1, 2, 3, 1]), 2);
        assert_eq!(inversions(&[0, 1, 2, 3, 2]), 1);
        assert_eq!(inversions(&[0, 1, 2, 3, 3]), 0);
        assert_eq!(inversions(&[0, 1, 2, 3, 4]), 0);
        assert_eq!(inversions(&[0, 1, 2, 4, 3]), 1);
        assert_eq!(inversions(&[0, 1, 3, 0, 2]), 3);
        assert_eq!(inversions(&[0, 1, 3, 1, 2]), 2);
        assert_eq!(inversions(&[0, 1, 3, 2, 0]), 4);
        assert_eq!(inversions(&[0, 1, 3, 2, 1]), 3);
        assert_eq!(inversions(&[0, 1, 3, 2, 2]), 2);
        assert_eq!(inversions(&[0, 1, 3, 2, 3]), 1);
        assert_eq!(inversions(&[0, 1, 3, 2, 4]), 1);
        assert_eq!(inversions(&[0, 1, 3, 3, 2]), 2);
        assert_eq!(inversions(&[0, 1, 3, 4, 2]), 2);
        assert_eq!(inversions(&[0, 1, 4, 2, 3]), 2);
        assert_eq!(inversions(&[0, 1, 4, 3, 2]), 3);
        assert_eq!(inversions(&[0, 2, 0, 0, 1]), 3);
        assert_eq!(inversions(&[0, 2, 0, 1, 0]), 4);
        assert_eq!(inversions(&[0, 2, 0, 1, 1]), 3);
        assert_eq!(inversions(&[0, 2, 0, 1, 2]), 2);
        assert_eq!(inversions(&[0, 2, 0, 1, 3]), 2);
        assert_eq!(inversions(&[0, 2, 0, 2, 1]), 3);
        assert_eq!(inversions(&[0, 2, 0, 3, 1]), 3);
        assert_eq!(inversions(&[0, 2, 1, 0, 0]), 5);
        assert_eq!(inversions(&[0, 2, 1, 0, 1]), 4);
        assert_eq!(inversions(&[0, 2, 1, 0, 2]), 3);
        assert_eq!(inversions(&[0, 2, 1, 0, 3]), 3);
        assert_eq!(inversions(&[0, 2, 1, 1, 0]), 5);
        assert_eq!(inversions(&[0, 2, 1, 1, 1]), 3);
        assert_eq!(inversions(&[0, 2, 1, 1, 2]), 2);
        assert_eq!(inversions(&[0, 2, 1, 1, 3]), 2);
        assert_eq!(inversions(&[0, 2, 1, 2, 0]), 4);
        assert_eq!(inversions(&[0, 2, 1, 2, 1]), 3);
        assert_eq!(inversions(&[0, 2, 1, 2, 2]), 1);
        assert_eq!(inversions(&[0, 2, 1, 2, 3]), 1);
        assert_eq!(inversions(&[0, 2, 1, 3, 0]), 4);
        assert_eq!(inversions(&[0, 2, 1, 3, 1]), 3);
        assert_eq!(inversions(&[0, 2, 1, 3, 2]), 2);
        assert_eq!(inversions(&[0, 2, 1, 3, 3]), 1);
        assert_eq!(inversions(&[0, 2, 1, 3, 4]), 1);
        assert_eq!(inversions(&[0, 2, 1, 4, 3]), 2);
        assert_eq!(inversions(&[0, 2, 2, 0, 1]), 4);
        assert_eq!(inversions(&[0, 2, 2, 1, 0]), 5);
        assert_eq!(inversions(&[0, 2, 2, 1, 1]), 4);
        assert_eq!(inversions(&[0, 2, 2, 1, 2]), 2);
        assert_eq!(inversions(&[0, 2, 2, 1, 3]), 2);
        assert_eq!(inversions(&[0, 2, 2, 2, 1]), 3);
        assert_eq!(inversions(&[0, 2, 2, 3, 1]), 3);
        assert_eq!(inversions(&[0, 2, 3, 0, 1]), 4);
        assert_eq!(inversions(&[0, 2, 3, 1, 0]), 5);
        assert_eq!(inversions(&[0, 2, 3, 1, 1]), 4);
        assert_eq!(inversions(&[0, 2, 3, 1, 2]), 3);
        assert_eq!(inversions(&[0, 2, 3, 1, 3]), 2);
        assert_eq!(inversions(&[0, 2, 3, 1, 4]), 2);
        assert_eq!(inversions(&[0, 2, 3, 2, 1]), 4);
        assert_eq!(inversions(&[0, 2, 3, 3, 1]), 3);
        assert_eq!(inversions(&[0, 2, 3, 4, 1]), 3);
        assert_eq!(inversions(&[0, 2, 4, 1, 3]), 3);
        assert_eq!(inversions(&[0, 2, 4, 3, 1]), 4);
        assert_eq!(inversions(&[0, 3, 0, 1, 2]), 3);
        assert_eq!(inversions(&[0, 3, 0, 2, 1]), 4);
        assert_eq!(inversions(&[0, 3, 1, 0, 2]), 4);
        assert_eq!(inversions(&[0, 3, 1, 1, 2]), 3);
        assert_eq!(inversions(&[0, 3, 1, 2, 0]), 5);
        assert_eq!(inversions(&[0, 3, 1, 2, 1]), 4);
        assert_eq!(inversions(&[0, 3, 1, 2, 2]), 3);
        assert_eq!(inversions(&[0, 3, 1, 2, 3]), 2);
        assert_eq!(inversions(&[0, 3, 1, 2, 4]), 2);
        assert_eq!(inversions(&[0, 3, 1, 3, 2]), 3);
        assert_eq!(inversions(&[0, 3, 1, 4, 2]), 3);
        assert_eq!(inversions(&[0, 3, 2, 0, 1]), 5);
        assert_eq!(inversions(&[0, 3, 2, 1, 0]), 6);
        assert_eq!(inversions(&[0, 3, 2, 1, 1]), 5);
        assert_eq!(inversions(&[0, 3, 2, 1, 2]), 4);
        assert_eq!(inversions(&[0, 3, 2, 1, 3]), 3);
        assert_eq!(inversions(&[0, 3, 2, 1, 4]), 3);
        assert_eq!(inversions(&[0, 3, 2, 2, 1]), 5);
        assert_eq!(inversions(&[0, 3, 2, 3, 1]), 4);
        assert_eq!(inversions(&[0, 3, 2, 4, 1]), 4);
        assert_eq!(inversions(&[0, 3, 3, 1, 2]), 4);
        assert_eq!(inversions(&[0, 3, 3, 2, 1]), 5);
        assert_eq!(inversions(&[0, 3, 4, 1, 2]), 4);
        assert_eq!(inversions(&[0, 3, 4, 2, 1]), 5);
        assert_eq!(inversions(&[0, 4, 1, 2, 3]), 3);
        assert_eq!(inversions(&[0, 4, 1, 3, 2]), 4);
        assert_eq!(inversions(&[0, 4, 2, 1, 3]), 4);
        assert_eq!(inversions(&[0, 4, 2, 3, 1]), 5);
        assert_eq!(inversions(&[0, 4, 3, 1, 2]), 5);
        assert_eq!(inversions(&[0, 4, 3, 2, 1]), 6);
        assert_eq!(inversions(&[1, 0, 0, 0, 0]), 4);
        assert_eq!(inversions(&[1, 0, 0, 0, 1]), 3);
        assert_eq!(inversions(&[1, 0, 0, 0, 2]), 3);
        assert_eq!(inversions(&[1, 0, 0, 1, 0]), 4);
        assert_eq!(inversions(&[1, 0, 0, 1, 1]), 2);
        assert_eq!(inversions(&[1, 0, 0, 1, 2]), 2);
        assert_eq!(inversions(&[1, 0, 0, 2, 0]), 4);
        assert_eq!(inversions(&[1, 0, 0, 2, 1]), 3);
        assert_eq!(inversions(&[1, 0, 0, 2, 2]), 2);
        assert_eq!(inversions(&[1, 0, 0, 2, 3]), 2);
        assert_eq!(inversions(&[1, 0, 0, 3, 2]), 3);
        assert_eq!(inversions(&[1, 0, 1, 0, 0]), 5);
        assert_eq!(inversions(&[1, 0, 1, 0, 1]), 3);
        assert_eq!(inversions(&[1, 0, 1, 0, 2]), 3);
        assert_eq!(inversions(&[1, 0, 1, 1, 0]), 4);
        assert_eq!(inversions(&[1, 0, 1, 1, 1]), 1);
        assert_eq!(inversions(&[1, 0, 1, 1, 2]), 1);
        assert_eq!(inversions(&[1, 0, 1, 2, 0]), 4);
        assert_eq!(inversions(&[1, 0, 1, 2, 1]), 2);
        assert_eq!(inversions(&[1, 0, 1, 2, 2]), 1);
        assert_eq!(inversions(&[1, 0, 1, 2, 3]), 1);
        assert_eq!(inversions(&[1, 0, 1, 3, 2]), 2);
        assert_eq!(inversions(&[1, 0, 2, 0, 0]), 5);
        assert_eq!(inversions(&[1, 0, 2, 0, 1]), 4);
        assert_eq!(inversions(&[1, 0, 2, 0, 2]), 3);
        assert_eq!(inversions(&[1, 0, 2, 0, 3]), 3);
        assert_eq!(inversions(&[1, 0, 2, 1, 0]), 5);
        assert_eq!(inversions(&[1, 0, 2, 1, 1]), 3);
        assert_eq!(inversions(&[1, 0, 2, 1, 2]), 2);
        assert_eq!(inversions(&[1, 0, 2, 1, 3]), 2);
        assert_eq!(inversions(&[1, 0, 2, 2, 0]), 4);
        assert_eq!(inversions(&[1, 0, 2, 2, 1]), 3);
        assert_eq!(inversions(&[1, 0, 2, 2, 2]), 1);
        assert_eq!(inversions(&[1, 0, 2, 2, 3]), 1);
        assert_eq!(inversions(&[1, 0, 2, 3, 0]), 4);
        assert_eq!(inversions(&[1, 0, 2, 3, 1]), 3);
        assert_eq!(inversions(&[1, 0, 2, 3, 2]), 2);
        assert_eq!(inversions(&[1, 0, 2, 3, 3]), 1);
        assert_eq!(inversions(&[1, 0, 2, 3, 4]), 1);
        assert_eq!(inversions(&[1, 0, 2, 4, 3]), 2);
        assert_eq!(inversions(&[1, 0, 3, 0, 2]), 4);
        assert_eq!(inversions(&[1, 0, 3, 1, 2]), 3);
        assert_eq!(inversions(&[1, 0, 3, 2, 0]), 5);
        assert_eq!(inversions(&[1, 0, 3, 2, 1]), 4);
        assert_eq!(inversions(&[1, 0, 3, 2, 2]), 3);
        assert_eq!(inversions(&[1, 0, 3, 2, 3]), 2);
        assert_eq!(inversions(&[1, 0, 3, 2, 4]), 2);
        assert_eq!(inversions(&[1, 0, 3, 3, 2]), 3);
        assert_eq!(inversions(&[1, 0, 3, 4, 2]), 3);
        assert_eq!(inversions(&[1, 0, 4, 2, 3]), 3);
        assert_eq!(inversions(&[1, 0, 4, 3, 2]), 4);
        assert_eq!(inversions(&[1, 1, 0, 0, 0]), 6);
        assert_eq!(inversions(&[1, 1, 0, 0, 1]), 4);
        assert_eq!(inversions(&[1, 1, 0, 0, 2]), 4);
        assert_eq!(inversions(&[1, 1, 0, 1, 0]), 5);
        assert_eq!(inversions(&[1, 1, 0, 1, 1]), 2);
        assert_eq!(inversions(&[1, 1, 0, 1, 2]), 2);
        assert_eq!(inversions(&[1, 1, 0, 2, 0]), 5);
        assert_eq!(inversions(&[1, 1, 0, 2, 1]), 3);
        assert_eq!(inversions(&[1, 1, 0, 2, 2]), 2);
        assert_eq!(inversions(&[1, 1, 0, 2, 3]), 2);
        assert_eq!(inversions(&[1, 1, 0, 3, 2]), 3);
        assert_eq!(inversions(&[1, 1, 1, 0, 0]), 6);
        assert_eq!(inversions(&[1, 1, 1, 0, 1]), 3);
        assert_eq!(inversions(&[1, 1, 1, 0, 2]), 3);
        assert_eq!(inversions(&[1, 1, 1, 1, 0]), 4);
        assert_eq!(inversions(&[1, 1, 1, 2, 0]), 4);
        assert_eq!(inversions(&[1, 1, 2, 0, 0]), 6);
        assert_eq!(inversions(&[1, 1, 2, 0, 1]), 4);
        assert_eq!(inversions(&[1, 1, 2, 0, 2]), 3);
        assert_eq!(inversions(&[1, 1, 2, 0, 3]), 3);
        assert_eq!(inversions(&[1, 1, 2, 1, 0]), 5);
        assert_eq!(inversions(&[1, 1, 2, 2, 0]), 4);
        assert_eq!(inversions(&[1, 1, 2, 3, 0]), 4);
        assert_eq!(inversions(&[1, 1, 3, 0, 2]), 4);
        assert_eq!(inversions(&[1, 1, 3, 2, 0]), 5);
        assert_eq!(inversions(&[1, 2, 0, 0, 0]), 6);
        assert_eq!(inversions(&[1, 2, 0, 0, 1]), 5);
        assert_eq!(inversions(&[1, 2, 0, 0, 2]), 4);
        assert_eq!(inversions(&[1, 2, 0, 0, 3]), 4);
        assert_eq!(inversions(&[1, 2, 0, 1, 0]), 6);
        assert_eq!(inversions(&[1, 2, 0, 1, 1]), 4);
        assert_eq!(inversions(&[1, 2, 0, 1, 2]), 3);
        assert_eq!(inversions(&[1, 2, 0, 1, 3]), 3);
        assert_eq!(inversions(&[1, 2, 0, 2, 0]), 5);
        assert_eq!(inversions(&[1, 2, 0, 2, 1]), 4);
        assert_eq!(inversions(&[1, 2, 0, 2, 2]), 2);
        assert_eq!(inversions(&[1, 2, 0, 2, 3]), 2);
        assert_eq!(inversions(&[1, 2, 0, 3, 0]), 5);
        assert_eq!(inversions(&[1, 2, 0, 3, 1]), 4);
        assert_eq!(inversions(&[1, 2, 0, 3, 2]), 3);
        assert_eq!(inversions(&[1, 2, 0, 3, 3]), 2);
        assert_eq!(inversions(&[1, 2, 0, 3, 4]), 2);
        assert_eq!(inversions(&[1, 2, 0, 4, 3]), 3);
        assert_eq!(inversions(&[1, 2, 1, 0, 0]), 7);
        assert_eq!(inversions(&[1, 2, 1, 0, 1]), 5);
        assert_eq!(inversions(&[1, 2, 1, 0, 2]), 4);
        assert_eq!(inversions(&[1, 2, 1, 0, 3]), 4);
        assert_eq!(inversions(&[1, 2, 1, 1, 0]), 6);
        assert_eq!(inversions(&[1, 2, 1, 2, 0]), 5);
        assert_eq!(inversions(&[1, 2, 1, 3, 0]), 5);
        assert_eq!(inversions(&[1, 2, 2, 0, 0]), 6);
        assert_eq!(inversions(&[1, 2, 2, 0, 1]), 5);
        assert_eq!(inversions(&[1, 2, 2, 0, 2]), 3);
        assert_eq!(inversions(&[1, 2, 2, 0, 3]), 3);
        assert_eq!(inversions(&[1, 2, 2, 1, 0]), 6);
        assert_eq!(inversions(&[1, 2, 2, 2, 0]), 4);
        assert_eq!(inversions(&[1, 2, 2, 3, 0]), 4);
        assert_eq!(inversions(&[1, 2, 3, 0, 0]), 6);
        assert_eq!(inversions(&[1, 2, 3, 0, 1]), 5);
        assert_eq!(inversions(&[1, 2, 3, 0, 2]), 4);
        assert_eq!(inversions(&[1, 2, 3, 0, 3]), 3);
        assert_eq!(inversions(&[1, 2, 3, 0, 4]), 3);
        assert_eq!(inversions(&[1, 2, 3, 1, 0]), 6);
        assert_eq!(inversions(&[1, 2, 3, 2, 0]), 5);
        assert_eq!(inversions(&[1, 2, 3, 3, 0]), 4);
        assert_eq!(inversions(&[1, 2, 3, 4, 0]), 4);
        assert_eq!(inversions(&[1, 2, 4, 0, 3]), 4);
        assert_eq!(inversions(&[1, 2, 4, 3, 0]), 5);
        assert_eq!(inversions(&[1, 3, 0, 0, 2]), 5);
        assert_eq!(inversions(&[1, 3, 0, 1, 2]), 4);
        assert_eq!(inversions(&[1, 3, 0, 2, 0]), 6);
        assert_eq!(inversions(&[1, 3, 0, 2, 1]), 5);
        assert_eq!(inversions(&[1, 3, 0, 2, 2]), 4);
        assert_eq!(inversions(&[1, 3, 0, 2, 3]), 3);
        assert_eq!(inversions(&[1, 3, 0, 2, 4]), 3);
        assert_eq!(inversions(&[1, 3, 0, 3, 2]), 4);
        assert_eq!(inversions(&[1, 3, 0, 4, 2]), 4);
        assert_eq!(inversions(&[1, 3, 1, 0, 2]), 5);
        assert_eq!(inversions(&[1, 3, 1, 2, 0]), 6);
        assert_eq!(inversions(&[1, 3, 2, 0, 0]), 7);
        assert_eq!(inversions(&[1, 3, 2, 0, 1]), 6);
        assert_eq!(inversions(&[1, 3, 2, 0, 2]), 5);
        assert_eq!(inversions(&[1, 3, 2, 0, 3]), 4);
        assert_eq!(inversions(&[1, 3, 2, 0, 4]), 4);
        assert_eq!(inversions(&[1, 3, 2, 1, 0]), 7);
        assert_eq!(inversions(&[1, 3, 2, 2, 0]), 6);
        assert_eq!(inversions(&[1, 3, 2, 3, 0]), 5);
        assert_eq!(inversions(&[1, 3, 2, 4, 0]), 5);
        assert_eq!(inversions(&[1, 3, 3, 0, 2]), 5);
        assert_eq!(inversions(&[1, 3, 3, 2, 0]), 6);
        assert_eq!(inversions(&[1, 3, 4, 0, 2]), 5);
        assert_eq!(inversions(&[1, 3, 4, 2, 0]), 6);
        assert_eq!(inversions(&[1, 4, 0, 2, 3]), 4);
        assert_eq!(inversions(&[1, 4, 0, 3, 2]), 5);
        assert_eq!(inversions(&[1, 4, 2, 0, 3]), 5);
        assert_eq!(inversions(&[1, 4, 2, 3, 0]), 6);
        assert_eq!(inversions(&[1, 4, 3, 0, 2]), 6);
        assert_eq!(inversions(&[1, 4, 3, 2, 0]), 7);
        assert_eq!(inversions(&[2, 0, 0, 0, 1]), 4);
        assert_eq!(inversions(&[2, 0, 0, 1, 0]), 5);
        assert_eq!(inversions(&[2, 0, 0, 1, 1]), 4);
        assert_eq!(inversions(&[2, 0, 0, 1, 2]), 3);
        assert_eq!(inversions(&[2, 0, 0, 1, 3]), 3);
        assert_eq!(inversions(&[2, 0, 0, 2, 1]), 4);
        assert_eq!(inversions(&[2, 0, 0, 3, 1]), 4);
        assert_eq!(inversions(&[2, 0, 1, 0, 0]), 6);
        assert_eq!(inversions(&[2, 0, 1, 0, 1]), 5);
        assert_eq!(inversions(&[2, 0, 1, 0, 2]), 4);
        assert_eq!(inversions(&[2, 0, 1, 0, 3]), 4);
        assert_eq!(inversions(&[2, 0, 1, 1, 0]), 6);
        assert_eq!(inversions(&[2, 0, 1, 1, 1]), 4);
        assert_eq!(inversions(&[2, 0, 1, 1, 2]), 3);
        assert_eq!(inversions(&[2, 0, 1, 1, 3]), 3);
        assert_eq!(inversions(&[2, 0, 1, 2, 0]), 5);
        assert_eq!(inversions(&[2, 0, 1, 2, 1]), 4);
        assert_eq!(inversions(&[2, 0, 1, 2, 2]), 2);
        assert_eq!(inversions(&[2, 0, 1, 2, 3]), 2);
        assert_eq!(inversions(&[2, 0, 1, 3, 0]), 5);
        assert_eq!(inversions(&[2, 0, 1, 3, 1]), 4);
        assert_eq!(inversions(&[2, 0, 1, 3, 2]), 3);
        assert_eq!(inversions(&[2, 0, 1, 3, 3]), 2);
        assert_eq!(inversions(&[2, 0, 1, 3, 4]), 2);
        assert_eq!(inversions(&[2, 0, 1, 4, 3]), 3);
        assert_eq!(inversions(&[2, 0, 2, 0, 1]), 5);
        assert_eq!(inversions(&[2, 0, 2, 1, 0]), 6);
        assert_eq!(inversions(&[2, 0, 2, 1, 1]), 5);
        assert_eq!(inversions(&[2, 0, 2, 1, 2]), 3);
        assert_eq!(inversions(&[2, 0, 2, 1, 3]), 3);
        assert_eq!(inversions(&[2, 0, 2, 2, 1]), 4);
        assert_eq!(inversions(&[2, 0, 2, 3, 1]), 4);
        assert_eq!(inversions(&[2, 0, 3, 0, 1]), 5);
        assert_eq!(inversions(&[2, 0, 3, 1, 0]), 6);
        assert_eq!(inversions(&[2, 0, 3, 1, 1]), 5);
        assert_eq!(inversions(&[2, 0, 3, 1, 2]), 4);
        assert_eq!(inversions(&[2, 0, 3, 1, 3]), 3);
        assert_eq!(inversions(&[2, 0, 3, 1, 4]), 3);
        assert_eq!(inversions(&[2, 0, 3, 2, 1]), 5);
        assert_eq!(inversions(&[2, 0, 3, 3, 1]), 4);
        assert_eq!(inversions(&[2, 0, 3, 4, 1]), 4);
        assert_eq!(inversions(&[2, 0, 4, 1, 3]), 4);
        assert_eq!(inversions(&[2, 0, 4, 3, 1]), 5);
        assert_eq!(inversions(&[2, 1, 0, 0, 0]), 7);
        assert_eq!(inversions(&[2, 1, 0, 0, 1]), 6);
        assert_eq!(inversions(&[2, 1, 0, 0, 2]), 5);
        assert_eq!(inversions(&[2, 1, 0, 0, 3]), 5);
        assert_eq!(inversions(&[2, 1, 0, 1, 0]), 7);
        assert_eq!(inversions(&[2, 1, 0, 1, 1]), 5);
        assert_eq!(inversions(&[2, 1, 0, 1, 2]), 4);
        assert_eq!(inversions(&[2, 1, 0, 1, 3]), 4);
        assert_eq!(inversions(&[2, 1, 0, 2, 0]), 6);
        assert_eq!(inversions(&[2, 1, 0, 2, 1]), 5);
        assert_eq!(inversions(&[2, 1, 0, 2, 2]), 3);
        assert_eq!(inversions(&[2, 1, 0, 2, 3]), 3);
        assert_eq!(inversions(&[2, 1, 0, 3, 0]), 6);
        assert_eq!(inversions(&[2, 1, 0, 3, 1]), 5);
        assert_eq!(inversions(&[2, 1, 0, 3, 2]), 4);
        assert_eq!(inversions(&[2, 1, 0, 3, 3]), 3);
        assert_eq!(inversions(&[2, 1, 0, 3, 4]), 3);
        assert_eq!(inversions(&[2, 1, 0, 4, 3]), 4);
        assert_eq!(inversions(&[2, 1, 1, 0, 0]), 8);
        assert_eq!(inversions(&[2, 1, 1, 0, 1]), 6);
        assert_eq!(inversions(&[2, 1, 1, 0, 2]), 5);
        assert_eq!(inversions(&[2, 1, 1, 0, 3]), 5);
        assert_eq!(inversions(&[2, 1, 1, 1, 0]), 7);
        assert_eq!(inversions(&[2, 1, 1, 2, 0]), 6);
        assert_eq!(inversions(&[2, 1, 1, 3, 0]), 6);
        assert_eq!(inversions(&[2, 1, 2, 0, 0]), 7);
        assert_eq!(inversions(&[2, 1, 2, 0, 1]), 6);
        assert_eq!(inversions(&[2, 1, 2, 0, 2]), 4);
        assert_eq!(inversions(&[2, 1, 2, 0, 3]), 4);
        assert_eq!(inversions(&[2, 1, 2, 1, 0]), 7);
        assert_eq!(inversions(&[2, 1, 2, 2, 0]), 5);
        assert_eq!(inversions(&[2, 1, 2, 3, 0]), 5);
        assert_eq!(inversions(&[2, 1, 3, 0, 0]), 7);
        assert_eq!(inversions(&[2, 1, 3, 0, 1]), 6);
        assert_eq!(inversions(&[2, 1, 3, 0, 2]), 5);
        assert_eq!(inversions(&[2, 1, 3, 0, 3]), 4);
        assert_eq!(inversions(&[2, 1, 3, 0, 4]), 4);
        assert_eq!(inversions(&[2, 1, 3, 1, 0]), 7);
        assert_eq!(inversions(&[2, 1, 3, 2, 0]), 6);
        assert_eq!(inversions(&[2, 1, 3, 3, 0]), 5);
        assert_eq!(inversions(&[2, 1, 3, 4, 0]), 5);
        assert_eq!(inversions(&[2, 1, 4, 0, 3]), 5);
        assert_eq!(inversions(&[2, 1, 4, 3, 0]), 6);
        assert_eq!(inversions(&[2, 2, 0, 0, 1]), 6);
        assert_eq!(inversions(&[2, 2, 0, 1, 0]), 7);
        assert_eq!(inversions(&[2, 2, 0, 1, 1]), 6);
        assert_eq!(inversions(&[2, 2, 0, 1, 2]), 4);
        assert_eq!(inversions(&[2, 2, 0, 1, 3]), 4);
        assert_eq!(inversions(&[2, 2, 0, 2, 1]), 5);
        assert_eq!(inversions(&[2, 2, 0, 3, 1]), 5);
        assert_eq!(inversions(&[2, 2, 1, 0, 0]), 8);
        assert_eq!(inversions(&[2, 2, 1, 0, 1]), 7);
        assert_eq!(inversions(&[2, 2, 1, 0, 2]), 5);
        assert_eq!(inversions(&[2, 2, 1, 0, 3]), 5);
        assert_eq!(inversions(&[2, 2, 1, 1, 0]), 8);
        assert_eq!(inversions(&[2, 2, 1, 2, 0]), 6);
        assert_eq!(inversions(&[2, 2, 1, 3, 0]), 6);
        assert_eq!(inversions(&[2, 2, 2, 0, 1]), 6);
        assert_eq!(inversions(&[2, 2, 2, 1, 0]), 7);
        assert_eq!(inversions(&[2, 2, 3, 0, 1]), 6);
        assert_eq!(inversions(&[2, 2, 3, 1, 0]), 7);
        assert_eq!(inversions(&[2, 3, 0, 0, 1]), 6);
        assert_eq!(inversions(&[2, 3, 0, 1, 0]), 7);
        assert_eq!(inversions(&[2, 3, 0, 1, 1]), 6);
        assert_eq!(inversions(&[2, 3, 0, 1, 2]), 5);
        assert_eq!(inversions(&[2, 3, 0, 1, 3]), 4);
        assert_eq!(inversions(&[2, 3, 0, 1, 4]), 4);
        assert_eq!(inversions(&[2, 3, 0, 2, 1]), 6);
        assert_eq!(inversions(&[2, 3, 0, 3, 1]), 5);
        assert_eq!(inversions(&[2, 3, 0, 4, 1]), 5);
        assert_eq!(inversions(&[2, 3, 1, 0, 0]), 8);
        assert_eq!(inversions(&[2, 3, 1, 0, 1]), 7);
        assert_eq!(inversions(&[2, 3, 1, 0, 2]), 6);
        assert_eq!(inversions(&[2, 3, 1, 0, 3]), 5);
        assert_eq!(inversions(&[2, 3, 1, 0, 4]), 5);
        assert_eq!(inversions(&[2, 3, 1, 1, 0]), 8);
        assert_eq!(inversions(&[2, 3, 1, 2, 0]), 7);
        assert_eq!(inversions(&[2, 3, 1, 3, 0]), 6);
        assert_eq!(inversions(&[2, 3, 1, 4, 0]), 6);
        assert_eq!(inversions(&[2, 3, 2, 0, 1]), 7);
        assert_eq!(inversions(&[2, 3, 2, 1, 0]), 8);
        assert_eq!(inversions(&[2, 3, 3, 0, 1]), 6);
        assert_eq!(inversions(&[2, 3, 3, 1, 0]), 7);
        assert_eq!(inversions(&[2, 3, 4, 0, 1]), 6);
        assert_eq!(inversions(&[2, 3, 4, 1, 0]), 7);
        assert_eq!(inversions(&[2, 4, 0, 1, 3]), 5);
        assert_eq!(inversions(&[2, 4, 0, 3, 1]), 6);
        assert_eq!(inversions(&[2, 4, 1, 0, 3]), 6);
        assert_eq!(inversions(&[2, 4, 1, 3, 0]), 7);
        assert_eq!(inversions(&[2, 4, 3, 0, 1]), 7);
        assert_eq!(inversions(&[2, 4, 3, 1, 0]), 8);
        assert_eq!(inversions(&[3, 0, 0, 1, 2]), 4);
        assert_eq!(inversions(&[3, 0, 0, 2, 1]), 5);
        assert_eq!(inversions(&[3, 0, 1, 0, 2]), 5);
        assert_eq!(inversions(&[3, 0, 1, 1, 2]), 4);
        assert_eq!(inversions(&[3, 0, 1, 2, 0]), 6);
        assert_eq!(inversions(&[3, 0, 1, 2, 1]), 5);
        assert_eq!(inversions(&[3, 0, 1, 2, 2]), 4);
        assert_eq!(inversions(&[3, 0, 1, 2, 3]), 3);
        assert_eq!(inversions(&[3, 0, 1, 2, 4]), 3);
        assert_eq!(inversions(&[3, 0, 1, 3, 2]), 4);
        assert_eq!(inversions(&[3, 0, 1, 4, 2]), 4);
        assert_eq!(inversions(&[3, 0, 2, 0, 1]), 6);
        assert_eq!(inversions(&[3, 0, 2, 1, 0]), 7);
        assert_eq!(inversions(&[3, 0, 2, 1, 1]), 6);
        assert_eq!(inversions(&[3, 0, 2, 1, 2]), 5);
        assert_eq!(inversions(&[3, 0, 2, 1, 3]), 4);
        assert_eq!(inversions(&[3, 0, 2, 1, 4]), 4);
        assert_eq!(inversions(&[3, 0, 2, 2, 1]), 6);
        assert_eq!(inversions(&[3, 0, 2, 3, 1]), 5);
        assert_eq!(inversions(&[3, 0, 2, 4, 1]), 5);
        assert_eq!(inversions(&[3, 0, 3, 1, 2]), 5);
        assert_eq!(inversions(&[3, 0, 3, 2, 1]), 6);
        assert_eq!(inversions(&[3, 0, 4, 1, 2]), 5);
        assert_eq!(inversions(&[3, 0, 4, 2, 1]), 6);
        assert_eq!(inversions(&[3, 1, 0, 0, 2]), 6);
        assert_eq!(inversions(&[3, 1, 0, 1, 2]), 5);
        assert_eq!(inversions(&[3, 1, 0, 2, 0]), 7);
        assert_eq!(inversions(&[3, 1, 0, 2, 1]), 6);
        assert_eq!(inversions(&[3, 1, 0, 2, 2]), 5);
        assert_eq!(inversions(&[3, 1, 0, 2, 3]), 4);
        assert_eq!(inversions(&[3, 1, 0, 2, 4]), 4);
        assert_eq!(inversions(&[3, 1, 0, 3, 2]), 5);
        assert_eq!(inversions(&[3, 1, 0, 4, 2]), 5);
        assert_eq!(inversions(&[3, 1, 1, 0, 2]), 6);
        assert_eq!(inversions(&[3, 1, 1, 2, 0]), 7);
        assert_eq!(inversions(&[3, 1, 2, 0, 0]), 8);
        assert_eq!(inversions(&[3, 1, 2, 0, 1]), 7);
        assert_eq!(inversions(&[3, 1, 2, 0, 2]), 6);
        assert_eq!(inversions(&[3, 1, 2, 0, 3]), 5);
        assert_eq!(inversions(&[3, 1, 2, 0, 4]), 5);
        assert_eq!(inversions(&[3, 1, 2, 1, 0]), 8);
        assert_eq!(inversions(&[3, 1, 2, 2, 0]), 7);
        assert_eq!(inversions(&[3, 1, 2, 3, 0]), 6);
        assert_eq!(inversions(&[3, 1, 2, 4, 0]), 6);
        assert_eq!(inversions(&[3, 1, 3, 0, 2]), 6);
        assert_eq!(inversions(&[3, 1, 3, 2, 0]), 7);
        assert_eq!(inversions(&[3, 1, 4, 0, 2]), 6);
        assert_eq!(inversions(&[3, 1, 4, 2, 0]), 7);
        assert_eq!(inversions(&[3, 2, 0, 0, 1]), 7);
        assert_eq!(inversions(&[3, 2, 0, 1, 0]), 8);
        assert_eq!(inversions(&[3, 2, 0, 1, 1]), 7);
        assert_eq!(inversions(&[3, 2, 0, 1, 2]), 6);
        assert_eq!(inversions(&[3, 2, 0, 1, 3]), 5);
        assert_eq!(inversions(&[3, 2, 0, 1, 4]), 5);
        assert_eq!(inversions(&[3, 2, 0, 2, 1]), 7);
        assert_eq!(inversions(&[3, 2, 0, 3, 1]), 6);
        assert_eq!(inversions(&[3, 2, 0, 4, 1]), 6);
        assert_eq!(inversions(&[3, 2, 1, 0, 0]), 9);
        assert_eq!(inversions(&[3, 2, 1, 0, 1]), 8);
        assert_eq!(inversions(&[3, 2, 1, 0, 2]), 7);
        assert_eq!(inversions(&[3, 2, 1, 0, 3]), 6);
        assert_eq!(inversions(&[3, 2, 1, 0, 4]), 6);
        assert_eq!(inversions(&[3, 2, 1, 1, 0]), 9);
        assert_eq!(inversions(&[3, 2, 1, 2, 0]), 8);
        assert_eq!(inversions(&[3, 2, 1, 3, 0]), 7);
        assert_eq!(inversions(&[3, 2, 1, 4, 0]), 7);
        assert_eq!(inversions(&[3, 2, 2, 0, 1]), 8);
        assert_eq!(inversions(&[3, 2, 2, 1, 0]), 9);
        assert_eq!(inversions(&[3, 2, 3, 0, 1]), 7);
        assert_eq!(inversions(&[3, 2, 3, 1, 0]), 8);
        assert_eq!(inversions(&[3, 2, 4, 0, 1]), 7);
        assert_eq!(inversions(&[3, 2, 4, 1, 0]), 8);
        assert_eq!(inversions(&[3, 3, 0, 1, 2]), 6);
        assert_eq!(inversions(&[3, 3, 0, 2, 1]), 7);
        assert_eq!(inversions(&[3, 3, 1, 0, 2]), 7);
        assert_eq!(inversions(&[3, 3, 1, 2, 0]), 8);
        assert_eq!(inversions(&[3, 3, 2, 0, 1]), 8);
        assert_eq!(inversions(&[3, 3, 2, 1, 0]), 9);
        assert_eq!(inversions(&[3, 4, 0, 1, 2]), 6);
        assert_eq!(inversions(&[3, 4, 0, 2, 1]), 7);
        assert_eq!(inversions(&[3, 4, 1, 0, 2]), 7);
        assert_eq!(inversions(&[3, 4, 1, 2, 0]), 8);
        assert_eq!(inversions(&[3, 4, 2, 0, 1]), 8);
        assert_eq!(inversions(&[3, 4, 2, 1, 0]), 9);
        assert_eq!(inversions(&[4, 0, 1, 2, 3]), 4);
        assert_eq!(inversions(&[4, 0, 1, 3, 2]), 5);
        assert_eq!(inversions(&[4, 0, 2, 1, 3]), 5);
        assert_eq!(inversions(&[4, 0, 2, 3, 1]), 6);
        assert_eq!(inversions(&[4, 0, 3, 1, 2]), 6);
        assert_eq!(inversions(&[4, 0, 3, 2, 1]), 7);
        assert_eq!(inversions(&[4, 1, 0, 2, 3]), 5);
        assert_eq!(inversions(&[4, 1, 0, 3, 2]), 6);
        assert_eq!(inversions(&[4, 1, 2, 0, 3]), 6);
        assert_eq!(inversions(&[4, 1, 2, 3, 0]), 7);
        assert_eq!(inversions(&[4, 1, 3, 0, 2]), 7);
        assert_eq!(inversions(&[4, 1, 3, 2, 0]), 8);
        assert_eq!(inversions(&[4, 2, 0, 1, 3]), 6);
        assert_eq!(inversions(&[4, 2, 0, 3, 1]), 7);
        assert_eq!(inversions(&[4, 2, 1, 0, 3]), 7);
        assert_eq!(inversions(&[4, 2, 1, 3, 0]), 8);
        assert_eq!(inversions(&[4, 2, 3, 0, 1]), 8);
        assert_eq!(inversions(&[4, 2, 3, 1, 0]), 9);
        assert_eq!(inversions(&[4, 3, 0, 1, 2]), 7);
        assert_eq!(inversions(&[4, 3, 0, 2, 1]), 8);
        assert_eq!(inversions(&[4, 3, 1, 0, 2]), 8);
        assert_eq!(inversions(&[4, 3, 1, 2, 0]), 9);
        assert_eq!(inversions(&[4, 3, 2, 0, 1]), 9);
        assert_eq!(inversions(&[4, 3, 2, 1, 0]), 10);
    }
}
