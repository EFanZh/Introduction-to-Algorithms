pub struct DirectAddressDictionary<T> {
    // Invariant: For all `i` in `0..self.keys.len()`, `self.memory[self.keys[i]].1 == i`.
    memory: Vec<(Option<T>, usize)>,
    keys: Vec<usize>,
}

impl<T> Default for DirectAddressDictionary<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DirectAddressDictionary<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            keys: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: T) {
        if let Some((value_ref, key_index)) = self.memory.get_mut(key) {
            if self.keys.get(*key_index) == Some(&key) {
                *value_ref = Some(value);
            } else {
                // Garbage location.

                *value_ref = Some(value);
                *key_index = self.keys.len();

                self.keys.push(key);
            }
        } else {
            // Memory is not enough.

            self.memory.resize_with(key + 1, Default::default);

            self.memory[key] = (Some(value), self.keys.len());

            self.keys.push(key);
        }
    }

    pub fn delete(&mut self, key: usize) {
        let (value, key_index) = self.memory.get_mut(key).unwrap();

        assert_eq!(self.keys.get(*key_index), Some(&key));

        // Clear value.

        *value = None;

        // Remove `key` from `self.keys`.

        self.keys.swap_remove(*key_index);

        if let Some(moved_key) = self.keys.get(*key_index) {
            // The last entry in `self.keys` is moved to `*key_index`, update the memory accordingly.

            self.memory[*moved_key].1 = *key_index;
        }
    }

    #[must_use]
    pub fn search(&self, key: usize) -> Option<&T> {
        if let Some((value, key_index)) = self.memory.get(key) {
            if self.keys.get(*key_index) == Some(&key) {
                return value.as_ref();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::DirectAddressDictionary;

    #[test]
    fn test_direct_address_dictionary() {
        let mut dict = DirectAddressDictionary::new();

        assert!(dict.search(4).is_none());

        dict.insert(4, 5);

        assert_eq!(dict.search(4), Some(&5));

        dict.insert(4, 6);

        assert_eq!(dict.search(4), Some(&6));

        dict.insert(2, 7);

        assert_eq!(dict.search(2), Some(&7));
        assert_eq!(dict.search(4), Some(&6));

        dict.delete(4);

        assert_eq!(dict.search(2), Some(&7));
        assert!(dict.search(4).is_none());

        dict.delete(2);

        assert!(dict.search(2).is_none());
        assert!(dict.search(4).is_none());
    }
}
