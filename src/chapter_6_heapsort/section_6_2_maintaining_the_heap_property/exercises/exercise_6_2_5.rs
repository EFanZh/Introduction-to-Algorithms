use super::super::super::section_6_1_heaps;

pub fn max_heapify_iterative<T: Ord>(a: &mut [T], mut i: usize) {
    let heap_size = a.len();

    loop {
        let l = section_6_1_heaps::left(i);
        let r = section_6_1_heaps::right(i);
        let mut largest = if l < heap_size && a[l] > a[i] { l } else { i };

        if r < heap_size && a[r] > a[largest] {
            largest = r;
        }

        if largest == i {
            break;
        }

        a.swap(i, largest);

        i = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::tests::run_max_heapify_test_cases;

    #[test]
    fn test_max_heapify_iterative() {
        run_max_heapify_test_cases(super::max_heapify_iterative);
    }
}
