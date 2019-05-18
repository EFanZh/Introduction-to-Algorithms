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
    use super::super::super::super::super::chapter_2_getting_started::section_2_1_insertion_sort::insertion_sort;
    use super::super::super::super::super::chapter_2_getting_started::section_2_3_designing_algorithms::merge_sort;
    use super::super::super::super::super::chapter_6_heapsort::section_6_4_the_heapsort_algorithm::heapsort;
    use super::super::super::super::super::chapter_7_quicksort::section_7_1_description_of_quicksort::quicksort;
    use super::super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::make_comparison_sort_stable;

    #[test]
    fn test_make_comparison_sort_stable_insertion_sort() {
        run_all_sorting_tests(make_comparison_sort_stable(insertion_sort));
    }

    #[test]
    fn test_make_comparison_sort_stable_merge_sort() {
        run_all_sorting_tests(make_comparison_sort_stable(|a| merge_sort(a, 0, a.len())));
    }

    #[test]
    fn test_make_comparison_sort_stable_heapsort() {
        run_all_sorting_tests(make_comparison_sort_stable(heapsort));
    }

    #[test]
    fn test_make_comparison_sort_stable_quicksort() {
        run_all_sorting_tests(make_comparison_sort_stable(|a| quicksort(a, 0, a.len())));
    }
}
