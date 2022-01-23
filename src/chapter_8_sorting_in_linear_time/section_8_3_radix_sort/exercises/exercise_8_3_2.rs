pub fn make_comparison_sort_stable<T: Clone + Ord, F: FnMut(&mut [(T, usize)])>(mut f: F) -> impl FnMut(&mut [T]) {
    move |a| {
        let mut b = a.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect::<Vec<_>>();

        f(&mut b);

        for (lhs, (rhs, _)) in a.iter_mut().zip(b) {
            *lhs = rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_2_getting_started::section_2_1_insertion_sort;
    use crate::chapter_2_getting_started::section_2_3_designing_algorithms;
    use crate::chapter_6_heapsort::section_6_4_the_heapsort_algorithm;
    use crate::chapter_7_quicksort::section_7_1_description_of_quicksort;
    use crate::test_utilities;

    #[test]
    fn test_make_comparison_sort_stable_insertion_sort() {
        test_utilities::run_all_sorting_tests(super::make_comparison_sort_stable(
            section_2_1_insertion_sort::insertion_sort,
        ));
    }

    #[test]
    fn test_make_comparison_sort_stable_merge_sort() {
        test_utilities::run_all_sorting_tests(super::make_comparison_sort_stable(|a| {
            section_2_3_designing_algorithms::merge_sort(a, 0, a.len());
        }));
    }

    #[test]
    fn test_make_comparison_sort_stable_heapsort() {
        test_utilities::run_all_sorting_tests(super::make_comparison_sort_stable(
            section_6_4_the_heapsort_algorithm::heapsort,
        ));
    }

    #[test]
    fn test_make_comparison_sort_stable_quicksort() {
        test_utilities::run_all_sorting_tests(super::make_comparison_sort_stable(|a| {
            section_7_1_description_of_quicksort::quicksort(a, 0, a.len());
        }));
    }
}
