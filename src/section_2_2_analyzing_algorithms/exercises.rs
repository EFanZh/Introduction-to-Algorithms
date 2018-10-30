pub mod exercise_2_2_2 {
    pub fn selection_sort<T: Ord + Clone>(a: &mut [T]) {
        let n = a.len();

        if n > 1 {
            for i in 0..n - 1 {
                let mut min_index = i;

                for j in i..n {
                    if a[j] < a[min_index] {
                        min_index = j;
                    }
                }

                a.swap(i, min_index);
            }
        }
    }
}
