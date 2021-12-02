use super::super::super::section_6_1_heaps::parent;

pub fn heap_increase_key<T: Ord>(a: &mut [T], mut i: usize, key: T) {
    assert!(key >= a[i], "new key is smaller than current key");

    while i > 0 && a[parent(i)] < key {
        // This simulates A[i] = A[Parent(i)]. Typically, swapping is faster than assignment in Rust.

        a.swap(i, parent(i));

        i = parent(i);
    }

    a[i] = key;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_heap_increase_key() {
        super::super::super::tests::run_heap_increase_key_test(super::heap_increase_key);
    }
}
