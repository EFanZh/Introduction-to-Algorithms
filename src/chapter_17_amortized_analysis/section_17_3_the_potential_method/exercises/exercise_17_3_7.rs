use crate::chapter_9_medians_and_order_statistics::section_9_2_selection_in_expected_linear_time;

pub struct DynamicMultiset<T> {
    data: Vec<T>,
}

impl<T: Ord> Default for DynamicMultiset<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> DynamicMultiset<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn delete_larger_half(&mut self) {
        let length = self.data.len();
        let half_length = length / 2;

        // Should use the `select` function instead of `randomized_select` function, but currently, `select` requires
        // `Clone` trait, which I don’t want to enforce, so I use `randomized_select` here.

        section_9_2_selection_in_expected_linear_time::randomized_select(&mut self.data, 0, length, half_length);

        self.data.truncate(half_length);
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicMultiset;

    #[test]
    fn test_dynamic_multiset() {
        fn assert_same_elements<T: Ord>(actual: &[T], expected: &[T]) {
            // TODO: assert!(expected.is_sorted());

            let mut actual = actual.iter().collect::<Box<_>>();

            actual.sort();

            assert!(actual.iter().copied().eq(expected.iter()));
        }

        let mut set = DynamicMultiset::new();

        assert_same_elements(set.as_slice(), &[]);

        set.insert(4);

        assert_same_elements(set.as_slice(), &[4]);

        set.insert(2);

        assert_same_elements(set.as_slice(), &[2, 4]);

        set.insert(3);

        assert_same_elements(set.as_slice(), &[2, 3, 4]);

        set.insert(8);
        set.insert(1);
        set.insert(6);

        assert_same_elements(set.as_slice(), &[1, 2, 3, 4, 6, 8]);

        set.delete_larger_half();

        assert_same_elements(set.as_slice(), &[1, 2, 3]);

        set.delete_larger_half();

        assert_same_elements(set.as_slice(), &[1]);
    }
}
