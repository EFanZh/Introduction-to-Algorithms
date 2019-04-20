pub fn merge<T: Clone + Ord>(a: &mut [T], p: usize, q: usize, r: usize) {
    let copied = a[p..r].to_vec();
    let (left, right) = copied.split_at(q - p);

    let mut i = 0;
    let mut j = 0;

    for k in p..r {
        if i < left.len() {
            if j < right.len() {
                if left[i] <= right[j] {
                    a[k] = left[i].clone();

                    i += 1;
                } else {
                    a[k] = right[j].clone();

                    j += 1;
                }
            } else {
                (&mut a[k..r]).clone_from_slice(&left[i..]);

                break;
            }
        } else {
            (&mut a[k..r]).clone_from_slice(&right[j..]);

            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;
    use crate::test_utilities::run_all_sorting_tests;

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

    #[test]
    fn test_merge_sort_2() {
        run_all_sorting_tests(merge_sort_helper_2);
    }
}
