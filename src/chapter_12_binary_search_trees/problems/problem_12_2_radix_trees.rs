use std::collections::BTreeMap;

pub struct RadixTree<T> {
    has_value: bool,
    children: BTreeMap<T, Self>,
}

impl<T: Ord> Default for RadixTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> RadixTree<T> {
    pub fn new() -> Self {
        Self {
            has_value: false,
            children: BTreeMap::new(),
        }
    }

    pub fn insert<I: IntoIterator<Item = T>>(&mut self, key: I) {
        let mut node = self;

        for item in key {
            node = node.children.entry(item).or_default()
        }

        node.has_value = true;
    }

    fn for_each_helper<'a, F: FnMut(&[&T])>(&'a self, buffer: &mut Vec<&'a T>, f: &mut F) {
        if self.has_value {
            f(buffer);
        }

        for (key, value) in &self.children {
            buffer.push(key);
            value.for_each_helper(buffer, f);
            buffer.pop();
        }
    }

    pub fn for_each<F: FnMut(&[&T])>(&self, mut f: F) {
        self.for_each_helper(&mut Vec::new(), &mut f);
    }
}

#[cfg(test)]
mod tests {
    use super::RadixTree;

    #[test]
    fn test_radix_tree_sorting() {
        let mut tree = RadixTree::new();

        tree.insert(vec![1, 0, 1, 1]);
        tree.insert(vec![1, 0]);
        tree.insert(vec![0, 1, 1]);
        tree.insert(vec![1, 0, 0]);
        tree.insert(vec![0]);

        let mut sorted_sequences = Vec::new();

        tree.for_each(|s| sorted_sequences.push(s.iter().map(|c| **c).collect::<Vec<_>>()));

        assert_eq!(
            sorted_sequences,
            vec![vec![0], vec![0, 1, 1], vec![1, 0], vec![1, 0, 0], vec![1, 0, 1, 1]]
        );
    }
}
