use super::section_6_2_maintaining_the_heap_property;
use super::section_6_3_building_a_heap;

// Heapsort(A)
// 1  Build-Max-Heap(A)
// 2  for i = A.length downto 2
// 3      exchange A[1] with A[i]
// 4      A.heap-size = A.heap-size - 1
// 5      Max-Heapify(A, 1)

pub fn heapsort<T: Ord>(a: &mut [T]) {
    section_6_3_building_a_heap::build_max_heap(a);

    for i in (1..a.len()).rev() {
        a.swap(0, i);

        section_6_2_maintaining_the_heap_property::max_heapify(&mut a[..i], 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    #[test]
    fn test_heapsort() {
        test_utilities::run_all_sorting_tests(super::heapsort);
    }
}
