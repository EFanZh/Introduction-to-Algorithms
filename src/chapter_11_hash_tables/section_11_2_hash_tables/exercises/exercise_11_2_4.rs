const INVALID_INDEX: usize = usize::max_value();

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

enum Slot<T> {
    Free { prev: usize, next: usize },
    Occupied { value: T, next: usize },
}

pub struct HashTable<T> {
    memory: Box<[Slot<T>]>,
    first_free_slot: usize,
}

impl<T: Hash + Eq> HashTable<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory: (0..capacity)
                .map(|i| Slot::Free {
                    prev: if i == 0 { INVALID_INDEX } else { i - 1 },
                    next: if i + 1 == capacity { INVALID_INDEX } else { i + 1 },
                })
                .collect(),
            first_free_slot: 0,
        }
    }

    fn hash(x: &T, memory_size: usize) -> usize {
        let mut hasher = DefaultHasher::new();

        x.hash(&mut hasher);

        (hasher.finish() as usize) % memory_size
    }

    fn allocate(&mut self) -> usize {
        let result = self.first_free_slot;

        if let Some(Slot::Free { next, .. }) = self.memory.get(self.first_free_slot) {
            let next = *next;

            if let Some(Slot::Free { prev, .. }) = self.memory.get_mut(next) {
                *prev = INVALID_INDEX;
            }

            self.first_free_slot = next;
        } else {
            panic!("No more memory");
        }

        result
    }

    fn free(&mut self, index: usize) {
        self.memory[index] = Slot::Free {
            prev: INVALID_INDEX,
            next: self.first_free_slot,
        };

        if let Some(Slot::Free { prev, .. }) = self.memory.get_mut(self.first_free_slot) {
            *prev = index;
        }

        self.first_free_slot = index;
    }

    pub fn insert(&mut self, x: T) {
        let memory_size = self.memory.len();
        let hash_value = Self::hash(&x, memory_size);

        match self.memory[hash_value] {
            Slot::Free { prev, next } => {
                if let Some(Slot::Free { next: prev_next, .. }) = self.memory.get_mut(prev) {
                    *prev_next = next;
                } else {
                    self.first_free_slot = next;
                }

                if let Some(Slot::Free { prev: next_prev, .. }) = self.memory.get_mut(next) {
                    *next_prev = prev;
                }

                self.memory[hash_value] = Slot::Occupied {
                    value: x,
                    next: INVALID_INDEX,
                };
            }
            Slot::Occupied {
                value: ref old_value,
                next: ref mut old_next,
            } => {
                let old_hash = Self::hash(old_value, memory_size);

                if old_hash == hash_value {
                    // This slot has the correct hash value.

                    // We know the next call of `self.allocate()` will return `self.first_free_slot`, so we set the
                    // `next` value to the slow index to be allocated. We can not call `self.allocate()` first because
                    // there will be borrow checker problem.

                    let old_next = mem::replace(old_next, self.first_free_slot);

                    self.memory[self.allocate()] = Slot::Occupied {
                        value: x,
                        next: old_next,
                    };
                } else {
                    // This slot contains hash value owned by some other slot.

                    let new_slot_index = self.allocate();

                    self.memory.swap(hash_value, new_slot_index);

                    // Fix the `next` value of the previous element of the moved element.

                    let mut i = old_hash;

                    while let Slot::Occupied { next, .. } = &mut self.memory[i] {
                        if *next == hash_value {
                            *next = new_slot_index;

                            break;
                        } else {
                            i = *next;
                        }
                    }

                    self.memory[hash_value] = Slot::Occupied {
                        value: x,
                        next: INVALID_INDEX,
                    };
                }
            }
        }
    }

    pub fn search(&self, x: &T) -> Option<&T> {
        let mut i = Self::hash(&x, self.memory.len());

        while let Some(Slot::Occupied { value, next }) = self.memory.get(i) {
            if value == x {
                return Some(&value);
            } else {
                i = *next;
            }
        }

        None
    }

    pub fn delete(&mut self, x: &T) {
        let hash_value = Self::hash(&x, self.memory.len());

        if let Slot::Occupied { ref value, next } = self.memory[hash_value] {
            if value == x {
                // The head element is the element to delete.

                if next == INVALID_INDEX {
                    self.free(hash_value);
                } else {
                    self.memory.swap(hash_value, next);
                    self.free(next);
                }
            } else {
                let mut i = hash_value;
                let mut j = next;

                while let Some(Slot::Occupied { value, next }) = self.memory.get(j) {
                    let next = *next;

                    if value == x {
                        if let Some(Slot::Occupied { next: prev_next, .. }) = self.memory.get_mut(i) {
                            *prev_next = next;
                        } else {
                            unreachable!();
                        }

                        self.free(j);
                    } else {
                        i = j;
                        j = next;
                    }
                }
            }
        } else {
            unreachable!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HashTable;

    enum HashTableOperation {
        Insert(i32),
        Delete(i32),
        Inspect(Vec<i32>, Vec<i32>),
    }

    #[test]
    fn test_hash_table() {
        use HashTableOperation::{Delete, Insert, Inspect};

        let test_case = vec![
            Inspect(vec![], vec![2, 3, 4, 5]),
            Insert(2),
            Inspect(vec![2], vec![3, 4, 5]),
            Insert(3),
            Inspect(vec![2, 3], vec![4, 5]),
            Insert(4),
            Inspect(vec![2, 3, 4], vec![5]),
            Insert(5),
            Inspect(vec![2, 3, 4, 5], vec![]),
            Delete(3),
            Inspect(vec![2, 4, 5], vec![3]),
            Delete(5),
            Inspect(vec![2, 4], vec![3, 5]),
            Delete(4),
            Inspect(vec![2], vec![3, 4, 5]),
            Delete(2),
            Inspect(vec![], vec![2, 3, 4, 5]),
        ];

        let mut hash_table = HashTable::with_capacity(4);

        for operation in test_case {
            match operation {
                Insert(value) => hash_table.insert(value),
                Delete(value) => hash_table.delete(&value),
                Inspect(contains, not_contains) => {
                    for value in contains {
                        assert_eq!(hash_table.search(&value), Some(&value));
                    }

                    for value in not_contains {
                        assert_eq!(hash_table.search(&value), None);
                    }
                }
            }
        }
    }
}
