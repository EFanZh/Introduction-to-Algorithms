use super::{super::section_2_3_designing_algorithms::merge_sort, utilities::run_all_sorting_tests};
use crate::test::Bencher;

fn merge_sort_helper<T: Ord + Clone>(a: &mut [T]) {
    merge_sort(a, 0, a.len());
}

#[bench]
fn test_merge_sort(b: &mut Bencher) {
    b.iter(|| run_all_sorting_tests(merge_sort_helper));
}

mod exercises {
    mod exercise_2_3_2 {
        use super::super::{
            super::{
                super::section_2_3_designing_algorithms::exercises::exercise_2_3_2::merge,
                utilities::run_all_sorting_tests,
            },
            Bencher,
        };

        fn merge_sort_helper_2<T: Ord + Clone>(a: &mut [T]) {
            pub fn merge_sort_2<T: Clone + Ord>(a: &mut [T], p: usize, r: usize) {
                if r - p > 1 {
                    let q = p + (r - p) / 2;

                    merge_sort_2(a, p, q);
                    merge_sort_2(a, q, r);
                    merge(a, p, q, r);
                }
            }

            merge_sort_2(a, 0, a.len());
        }

        #[bench]
        fn test_merge_sort_2(b: &mut Bencher) {
            b.iter(|| run_all_sorting_tests(merge_sort_helper_2));
        }
    }

    mod exercise_2_3_4 {
        use super::super::{
            super::{
                super::section_2_3_designing_algorithms::exercises::exercise_2_3_4::insertion_sort_recursive,
                utilities::run_all_sorting_tests,
            },
            Bencher,
        };

        #[bench]
        fn test_insertion_sort_recursive(b: &mut Bencher) {
            b.iter(|| run_all_sorting_tests(insertion_sort_recursive));
        }
    }
}
