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
    use super::insertion_sort_reversed;
    use crate::test_utilities::test_sort;

    #[test]
    fn test_exercise_insertion_sort_reversed() {
        test_sort(insertion_sort_reversed, [] as [i32; 0], []);

        test_sort(insertion_sort_reversed, [0], [0]);

        test_sort(insertion_sort_reversed, [0, 0], [0, 0]);
        test_sort(insertion_sort_reversed, [0, 1], [1, 0]);
        test_sort(insertion_sort_reversed, [1, 0], [1, 0]);

        test_sort(insertion_sort_reversed, [0, 0, 0], [0, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 1], [1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 0], [1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 1], [1, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 2], [2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 1], [2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 0], [1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 1], [1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 2], [2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 0], [1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 0], [2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 1], [2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 0], [2, 1, 0]);

        test_sort(insertion_sort_reversed, [0, 0, 0, 0], [0, 0, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 0, 1], [1, 0, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 1, 0], [1, 0, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 1, 1], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 1, 2], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 0, 2, 1], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 0, 0], [1, 0, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 0, 1], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 0, 2], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 1, 0], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 1, 1], [1, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 1, 2], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 2, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 2, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 2, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 2, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 1, 3, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 0, 1], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 1, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 1, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 1, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 1, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 2, 1], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 2, 3, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 3, 1, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [0, 3, 2, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 0, 0], [1, 0, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 0, 1], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 0, 2], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 1, 0], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 1, 1], [1, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 1, 2], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 2, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 2, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 2, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 2, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 0, 3, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 0, 0], [1, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 0, 1], [1, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 0, 2], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 1, 0], [1, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 1, 2, 0], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 0, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 0, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 0, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 0, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 1, 0], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 2, 0], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 2, 3, 0], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 3, 0, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [1, 3, 2, 0], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 0, 1], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 1, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 1, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 1, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 1, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 2, 1], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 0, 3, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 0, 0], [2, 1, 0, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 0, 1], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 0, 2], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 0, 3], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 1, 0], [2, 1, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 2, 0], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 1, 3, 0], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 2, 0, 1], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 2, 1, 0], [2, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 3, 0, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [2, 3, 1, 0], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 0, 1, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 0, 2, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 1, 0, 2], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 1, 2, 0], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 2, 0, 1], [3, 2, 1, 0]);
        test_sort(insertion_sort_reversed, [3, 2, 1, 0], [3, 2, 1, 0]);
    }
}
