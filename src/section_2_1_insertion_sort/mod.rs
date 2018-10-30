// Insertion-Sort(A)
//
// 1  for j = 2 to A.length
// 2      key = A[j]
// 3      // Insert A[j] into the sorted sequence A[1..j - 1].
// 4      i = j - 1
// 5      while i > 0 and A[i] > key
// 6          A[i + 1] = A[i]
// 7          i = i - 1
// 8      A[i + 1] = key

pub fn insertion_sort<T: Ord + Clone>(a: &mut [T]) {
    for j in 1..a.len() {
        let key = a[j].clone();

        // Insert `a[j]` into the sorted sequence `a[0..j]`.

        let mut i = j - 1;

        while i < a.len() && a[i] > key {
            a[i + 1] = a[i].clone();
            i = i.wrapping_sub(1);
        }

        a[i.wrapping_add(1)] = key;
    }
}

pub mod extra {
    pub fn insertion_sort_tail_recursive<T: Ord + Clone>(a: &mut [T]) {
        fn insert_key<T: Ord + Clone>(a: &mut [T], key: T) {
            let free_slot_index = a.len() - 1;

            if free_slot_index > 0 && a[free_slot_index - 1] > key {
                a[free_slot_index] = a[free_slot_index - 1].clone();

                insert_key(&mut a[..free_slot_index], key);
            } else {
                a[free_slot_index] = key;
            }
        }

        fn insertion_sort_helper<T: Ord + Clone>(a: &mut [T], num_sorted: usize) {
            if num_sorted < a.len() {
                let key = a[num_sorted].clone();
                let next_separator = num_sorted + 1;

                insert_key(&mut a[0..next_separator], key);

                insertion_sort_helper(a, next_separator);
            }
        }

        insertion_sort_helper(a, 0);
    }
}

pub mod exercises {
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
}
