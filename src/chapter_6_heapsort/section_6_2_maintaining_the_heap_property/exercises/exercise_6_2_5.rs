use super::super::super::section_6_1_heaps::{left, right};

pub fn max_heapify_iterative<T: Ord>(a: &mut [T], mut i: usize) {
    let heap_size = a.len();

    loop {
        let l = left(i);
        let r = right(i);
        let mut largest = if l < heap_size && a[l] > a[i] { l } else { i };

        if r < heap_size && a[r] > a[largest] {
            largest = r
        }

        if largest == i {
            break;
        } else {
            a.swap(i, largest);

            i = largest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_max_heapify_test_cases;
    use super::max_heapify_iterative;

    #[test]
    fn test_max_heapify_iterative() {
        run_max_heapify_test_cases(max_heapify_iterative);
    }
}
