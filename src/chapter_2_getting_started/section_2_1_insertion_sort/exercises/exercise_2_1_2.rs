pub fn insertion_sort_reversed<T: Ord + Clone>(a: &mut [T]) {
    for j in 1..a.len() {
        let key = a[j].clone();

        // Insert `a[j]` into the sorted sequence `a[0..j]`.

        let mut i = j - 1;

        while i < a.len() && a[i] < key {
            a[i + 1] = a[i].clone();
            i = i.wrapping_sub(1);
        }

        a[i.wrapping_add(1)] = key;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exercise_insertion_sort_reversed() {
        super::super::super::super::super::test_utilities::run_all_reverse_sorting_tests(
            super::insertion_sort_reversed,
        );
    }
}
