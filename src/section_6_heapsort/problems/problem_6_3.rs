use super::super::super::utilities::Infinitable;

pub struct YoungTableau<T: Ord> {
    data: Box<[Infinitable<T>]>,
    num_columns: usize,
}

impl<T: Ord> YoungTableau<T> {
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        YoungTableau {
            data: (0..num_columns * num_rows).map(|_| Infinitable::Infinity).collect(),
            num_columns,
        }
    }

    fn get_num_rows(&self) -> usize {
        self.data.len() / self.num_columns
    }

    fn get_data_index(&self, i: usize, j: usize) -> usize {
        self.num_columns * i + j
    }

    fn get_element(&self, i: usize, j: usize) -> &Infinitable<T> {
        &self.data[self.get_data_index(i, j)]
    }

    fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        self.data.swap(self.get_data_index(i1, j1), self.get_data_index(i2, j2))
    }

    fn young_tableaufy(&mut self, mut i: usize, mut j: usize) {
        let num_rows = self.get_num_rows();
        let num_columns = self.num_columns;

        loop {
            let mut smallest_i = i;
            let mut smallest_j = j;

            if i + 1 < num_rows && self.get_element(i + 1, j) < self.get_element(smallest_i, smallest_j) {
                smallest_i = i + 1;
                smallest_j = j;
            }

            if j + 1 < num_columns && self.get_element(i, j + 1) < self.get_element(smallest_i, smallest_j) {
                smallest_i = i;
                smallest_j = j + 1;
            }

            if smallest_i != i || smallest_j != j {
                self.swap(i, j, smallest_i, smallest_j);

                i = smallest_i;
                j = smallest_j;
            } else {
                break;
            }
        }
    }

    pub fn extract_min(&mut self) -> T {
        let value = self.data[0].replace_with_infinity().unwrap();

        self.young_tableaufy(0, 0);

        value
    }

    pub fn insert(&mut self, key: T) {
        self.data[self.data.len() - 1] = Infinitable::Finity(key);

        let mut i = self.get_num_rows() - 1;
        let mut j = self.num_columns - 1;

        loop {
            let mut largest_i = i;
            let mut largest_j = j;

            if i > 0 && self.get_element(i - 1, j) > self.get_element(largest_i, largest_j) {
                largest_i = i - 1;
                largest_j = j;
            }

            if j > 0 && self.get_element(i, j - 1) > self.get_element(largest_i, largest_j) {
                largest_i = i;
                largest_j = j - 1;
            }

            if largest_i != i || largest_j != j {
                self.swap(i, j, largest_i, largest_j);

                i = largest_i;
                j = largest_j;
            } else {
                break;
            }
        }
    }
}

pub fn young_tableau_sort<T: Ord + Clone>(a: &mut [T]) {
    let n = (a.len() as f64).sqrt().ceil() as usize;
    let mut young_tableau = YoungTableau::new(n, n);

    for value in a.iter() {
        young_tableau.insert(value.clone());
    }

    for value in a {
        *value = young_tableau.extract_min();
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::test_utilities::run_all_sorting_tests;
    use super::{young_tableau_sort, YoungTableau};

    #[test]
    fn test_young_tableau_insert_and_extract_min() {
        let mut young_tableau = YoungTableau::new(3, 4);

        young_tableau.insert(4);

        assert_eq!(young_tableau.extract_min(), 4);

        young_tableau.insert(4);
        young_tableau.insert(5);

        assert_eq!(young_tableau.extract_min(), 4);
        assert_eq!(young_tableau.extract_min(), 5);

        young_tableau.insert(5);
        young_tableau.insert(4);

        assert_eq!(young_tableau.extract_min(), 4);
        assert_eq!(young_tableau.extract_min(), 5);

        young_tableau.insert(1);
        young_tableau.insert(7);
        young_tableau.insert(4);
        young_tableau.insert(2);
        young_tableau.insert(9);
        young_tableau.insert(8);
        young_tableau.insert(8);
        young_tableau.insert(6);
        young_tableau.insert(6);
        young_tableau.insert(3);

        assert_eq!(young_tableau.extract_min(), 1);
        assert_eq!(young_tableau.extract_min(), 2);
        assert_eq!(young_tableau.extract_min(), 3);
        assert_eq!(young_tableau.extract_min(), 4);
        assert_eq!(young_tableau.extract_min(), 6);
        assert_eq!(young_tableau.extract_min(), 6);
        assert_eq!(young_tableau.extract_min(), 7);
        assert_eq!(young_tableau.extract_min(), 8);
        assert_eq!(young_tableau.extract_min(), 8);
        assert_eq!(young_tableau.extract_min(), 9);
    }

    #[test]
    fn test_young_tableau_sort() {
        run_all_sorting_tests(young_tableau_sort);
    }
}
