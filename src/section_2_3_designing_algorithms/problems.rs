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
