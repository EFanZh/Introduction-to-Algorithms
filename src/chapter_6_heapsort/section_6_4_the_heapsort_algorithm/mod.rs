use super::section_6_2_maintaining_the_heap_property::max_heapify;
use super::section_6_3_building_a_heap::build_max_heap;

// Heapsort(A)
// 1  Build-Max-Heap(A)
// 2  for i = A.length downto 2
// 3      exchange A[1] with A[i]
// 4      A.heap-size = A.heap-size - 1
// 5      Max-Heapify(A, 1)

pub fn heapsort<T: Ord>(a: &mut [T]) {
    build_max_heap(a);

    for i in (1..a.len()).rev() {
        a.swap(0, i);

        max_heapify(&mut a[..i], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::test_utilities::run_all_sorting_tests;
    use super::heapsort;

    #[test]
    fn test_heapsort() {
        run_all_sorting_tests(heapsort);
    }
}
