pub mod problem_2_2 {
    // Bubblesort(A)
    //
    // 1  for i = 1 to A.length - 1
    // 2      for j = A.length downto i + 1
    // 3          if A[j] < A[j - 1]
    // 4              exchange A[j] with A[j - 1]

    pub fn bubble_sort<T: Ord + Clone>(a: &mut [T]) {
        for i in 1..a.len() {
            for j in (i..a.len()).rev() {
                if a[j] < a[j - 1] {
                    a.swap(j, j - 1);
                }
            }
        }
    }
}

pub mod problem_2_3 {
    use std::ops::{Add, Mul};

    // 1  y = 0
    // 2  for i = n downto 0
    // 3      y = ai + x ⋅ y

    pub fn polynomial<T: Default>(a: &[T], x: &T) -> T
    where
        for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
    {
        let mut y: T = Default::default();

        for a_i in a.iter().rev() {
            y = a_i + &(x * &y);
        }

        y
    }

    pub fn polynomial_naive<T: Default + Clone>(a: &[T], x: &T) -> T
    where
        for<'a> &'a T: Add<Output = T> + Mul<Output = T>,
    {
        let mut y: T = Default::default();

        for (i, a_i) in a.iter().enumerate() {
            let mut p = a_i.clone();

            for _ in 0..i {
                p = &p * x;
            }

            y = &y + &p;
        }

        y
    }
}

pub mod problem_2_4 {
    pub fn inversions<T: Clone + Ord>(a: &[T]) -> usize {
        pub fn merge<T: Clone + Ord>(left: &[T], right: &[T]) -> (Vec<T>, usize) {
            let mut sorted = Vec::with_capacity(left.len() + right.len());
            let mut n = 0;

            let mut i = 0;
            let mut j = 0;

            loop {
                if left[i] <= right[j] {
                    sorted.push(left[i].clone());

                    // There are `j` elements in array `right` that is less than `left[i]`.
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
}
