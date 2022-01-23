pub fn inversions<T: Clone + Ord>(a: &[T]) -> usize {
    pub fn merge<T: Clone + Ord>(left: &[T], right: &[T]) -> (Vec<T>, usize) {
        let mut sorted = Vec::with_capacity(left.len() + right.len());
        let mut n = 0;

        let mut i = 0;
        let mut j = 0;

        loop {
            if left[i] <= right[j] {
                sorted.push(left[i].clone());

                // There are `j` elements in array `right` that are less than `left[i]`.
                n += j;

                i += 1;

                if i == left.len() {
                    sorted.extend_from_slice(&right[j..]);

                    break;
                }
            } else {
                sorted.push(right[j].clone());

                j += 1;

                if j == right.len() {
                    sorted.extend_from_slice(&left[i..]);

                    // For each element k in `left[i..]`, there are `j` elements that are less than k.
                    n += j * (left.len() - i);

                    break;
                }
            }
        }

        (sorted, n)
    }

    pub fn merge_sort<T: Clone + Ord>(a: &[T]) -> (Vec<T>, usize) {
        if a.len() > 1 {
            let (left, right) = a.split_at(a.len() / 2);
            let (left_sorted, inversions_left) = merge_sort(left);
            let (right_sorted, inversions_right) = merge_sort(right);
            let (sorted, inversions_between) = merge(&left_sorted, &right_sorted);

            (sorted, inversions_left + inversions_right + inversions_between)
        } else {
            (a.to_vec(), 0)
        }
    }

    merge_sort(a).1
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_inversions() {
        test_utilities::run_all_num_inversions_tests(super::inversions);
    }
}
