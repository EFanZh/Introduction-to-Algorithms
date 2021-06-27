pub struct DAryHeap<T> {
    data: Vec<T>,
    d: usize,
}

impl<T: Ord> DAryHeap<T> {
    #[must_use]
    pub fn new(d: usize) -> Self {
        DAryHeap { data: Vec::new(), d }
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / self.d
    }

    fn children(&self, i: usize) -> (usize, usize) {
        let start = i * self.d + 1;
        let end = start + self.d;

        (start, end)
    }

    fn max_heapify(&mut self, mut i: usize) {
        while let Some((j, child_value)) = {
            let (start, end) = self.children(i);

            self.data
                .iter()
                .enumerate()
                .take(end)
                .skip(start)
                .max_by_key(|(_, v)| *v)
        } {
            if *child_value > self.data[i] {
                self.data.swap(i, j);

                i = j;
            } else {
                break;
            }
        }
    }

    pub fn extract_max(&mut self) -> T {
        let result = self.data.swap_remove(0);

        self.max_heapify(0);

        result
    }

    fn fix_ancestors_unchecked(&mut self, mut i: usize) {
        while i > 0 && self.data[self.parent(i)] < self.data[i] {
            let parent = self.parent(i);

            self.data.swap(i, parent);

            i = parent;
        }
    }

    pub fn insert(&mut self, x: T) {
        self.data.push(x);

        self.fix_ancestors_unchecked(self.data.len() - 1);
    }

    pub fn increase_key(&mut self, i: usize, k: T) {
        if k < self.data[i] {
            panic!("new key is smaller than current key");
        }

        self.data[i] = k;

        self.fix_ancestors_unchecked(i);
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::assign_vec_from_iter;
    use super::DAryHeap;
    use std::iter;

    #[test]
    fn test_n_ary_heap_insert_and_extract_max() {
        let mut data = Vec::new();
        let mut sorted_data = Vec::new();

        for d in 1..4 {
            for n in 0..8 {
                assign_vec_from_iter(&mut data, 0..n);
                assign_vec_from_iter(&mut sorted_data, (0..n).rev());

                permutohedron::heap_recursive(&mut data, |sequence| {
                    let mut heap = DAryHeap::new(d);

                    for value in sequence {
                        heap.insert(value);
                    }

                    assert!(iter::repeat_with(|| heap.extract_max()).take(n).eq(sorted_data.iter()));
                });
            }
        }
    }

    #[test]
    fn test_d_ary_heap_increase_key() {
        for d in 1..4 {
            let mut heap = DAryHeap::new(d);

            heap.insert(1);
            heap.increase_key(0, 4);

            assert_eq!(heap.extract_max(), 4);

            heap.insert(1);
            heap.insert(2);
            heap.insert(3);
            heap.increase_key(0, 4);

            assert_eq!(heap.extract_max(), 4);

            heap.insert(1);
            heap.insert(2);
            heap.insert(3);
            heap.increase_key(1, 4);

            assert_eq!(heap.extract_max(), 4);

            heap.insert(1);
            heap.insert(2);
            heap.insert(3);
            heap.increase_key(2, 4);

            assert_eq!(heap.extract_max(), 4);
        }
    }
}
