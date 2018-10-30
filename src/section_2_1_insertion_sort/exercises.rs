pub mod exercise_2_1_2 {
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
}

pub mod exercise_2_1_3 {
    pub fn search<U, T: PartialEq<U>>(a: &[T], v: &U) -> Option<usize> {
        for (i, item) in a.iter().enumerate() {
            if *item == *v {
                return Some(i);
            }
        }

        None
    }
}

pub mod exercise_2_1_4 {
    pub fn add_binary(a: &[u8], b: &[u8], c: &mut [u8]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len() + 1, c.len());

        let n = a.len();
        let mut carry = 0;

        for i in 0..n {
            let sum = a[n - 1 - i] + b[n - 1 - i] + carry;

            c[n - i] = sum % 2;
            carry = sum / 2;
        }

        c[0] = carry;
    }
}
