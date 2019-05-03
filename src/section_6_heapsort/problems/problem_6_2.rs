pub struct NAryHeap<T> {
    data: Vec<T>,
    n: usize,
}

impl<T: Ord> NAryHeap<T> {
    pub fn new(n: usize) -> Self {
        NAryHeap { data: Vec::new(), n }
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / self.n
    }

    fn children(&self, i: usize) -> (usize, usize) {
        let start = i * self.n + 1;
        let end = start + self.n;

        (start, end)
    }

    fn max_heapify(&mut self, mut i: usize) {
        loop {
            let (start, end) = self.children(i);

            if let Some((j, child_value)) = self
                .data
                .iter()
                .enumerate()
                .take(end)
                .skip(start)
                .max_by_key(|(_, v)| *v)
            {
                if *child_value > self.data[i] {
                    self.data.swap(i, j);

                    i = j;
                } else {
                    break;
                }
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
    use super::NAryHeap;
    use permutohedron::heap_recursive;

    #[test]
    fn test_n_ary_heap_insert_and_extract_max() {
        let mut data = Vec::new();
        let mut sorted_data = Vec::new();

        for n in 1..4 {
            for num_nodes in 0..8 {
                assign_vec_from_iter(&mut data, 0..num_nodes);
                assign_vec_from_iter(&mut sorted_data, (0..num_nodes).rev());

                heap_recursive(&mut data, |sequence| {
                    let mut heap = NAryHeap::new(n);

                    for value in sequence {
                        heap.insert(value);
                    }

                    assert!((0..num_nodes).map(|_| heap.extract_max()).eq(sorted_data.iter()));
                });
            }
        }
    }

    #[test]
    fn test_n_ary_heap_increase_key() {
        for n in 1..4 {
            let mut heap = NAryHeap::new(n);

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
