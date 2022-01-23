use super::super::super::section_6_1_heaps;

pub fn min_heapify<T: Ord>(a: &mut [T], i: usize) {
    let heap_size = a.len();
    let l = section_6_1_heaps::left(i);
    let r = section_6_1_heaps::right(i);
    let mut smallest = if l < heap_size && a[l] < a[i] { l } else { i };

    if r < heap_size && a[r] < a[smallest] {
        smallest = r;
    }

    if smallest != i {
        a.swap(i, smallest);

        min_heapify(a, smallest);
    }
}

pub fn min_heapify_iterative<T: Ord>(a: &mut [T], mut i: usize) {
    let heap_size = a.len();

    loop {
        let l = section_6_1_heaps::left(i);
        let r = section_6_1_heaps::right(i);
        let mut smallest = if l < heap_size && a[l] < a[i] { l } else { i };

        if r < heap_size && a[r] < a[smallest] {
            smallest = r;
        }

        if smallest == i {
            break;
        }

        a.swap(i, smallest);

        i = smallest;
    }
}

#[cfg(test)]
mod tests {
    pub fn run_min_heapify_test_cases<F: Fn(&mut [i32], usize)>(f: F) {
        let run_single_test = |a: &mut [i32], i, b: &[i32]| {
            f(a, i);

            assert_eq!(a, b);
        };

        run_single_test(
            &mut [0, 12, 6, 2, 9, 7, 13, 14, 8, 15],
            1,
            &[0, 2, 6, 8, 9, 7, 13, 14, 12, 15],
        );
    }

    #[test]
    fn test_min_heapify() {
        run_min_heapify_test_cases(super::min_heapify);
    }

    #[test]
    fn test_min_heapify_iterative() {
        run_min_heapify_test_cases(super::min_heapify_iterative);
    }
}
