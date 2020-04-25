use std::iter;

pub struct DynamicTable<T> {
    slots: Box<[Option<T>]>,
    length: usize,
}

impl<T> Default for DynamicTable<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DynamicTable<T> {
    pub fn new() -> Self {
        Self {
            slots: Box::new([]),
            length: 0,
        }
    }

    fn allocate(length: usize) -> Box<[Option<T>]> {
        iter::repeat_with(|| None).take(length).collect::<Box<_>>()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn capacity(&self) -> usize {
        self.slots.len()
    }

    // Table-Insert(T, x)
    //
    //  1  if T.size == 0
    //  2      allocate T.table with 1 slot
    //  3      T.size = 1
    //  4  if T.num == T.size
    //  5      allocate new-table with 2 ⋅ T.size slots
    //  6      insert all items in T.table into new-table
    //  7      free T.table
    //  8      T.table = new-table
    //  9      T.size = 2 ⋅ T.size
    // 10  insert x into T.table
    // 11  T.num = T.num + 1

    pub fn push(&mut self, value: T) {
        if self.length == self.capacity() {
            let mut new_slots = Self::allocate((self.capacity() * 2).max(1));

            for (source, target) in self.slots.iter_mut().zip(new_slots.iter_mut()) {
                *target = source.take();
            }

            self.slots = new_slots;
        }

        self.slots[self.length] = Some(value);

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            1 => {
                self.length = 0;

                let result = self.slots[0].take();

                self.slots = Self::allocate(0);

                result
            }
            _ => {
                self.length -= 1;

                let result = self.slots[self.length].take();

                if self.length * 4 < self.capacity() {
                    let mut new_slots = Self::allocate(self.capacity() / 2);

                    for (source, target) in self.slots[..self.length].iter_mut().zip(new_slots.iter_mut()) {
                        *target = source.take();
                    }

                    self.slots = new_slots;
                }

                result
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + DoubleEndedIterator + ExactSizeIterator {
        self.slots[..self.length].iter().map(|slot| slot.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicTable;

    enum Operation {
        Push(i32, usize, usize),
        Pop(usize, usize),
    }

    #[test]
    fn test_dynamic_table() {
        use Operation::{Pop, Push};

        let mut table = DynamicTable::new();
        let mut control = Vec::new();

        assert!(table.is_empty());
        assert!(table.iter().eq(control.iter()));
        assert_eq!(table.capacity(), 0);

        let operations = &[
            Push(2, 1, 1),    //  1
            Push(3, 2, 2),    //  2
            Push(5, 3, 4),    //  3
            Push(7, 4, 4),    //  4
            Push(11, 5, 8),   //  5
            Push(13, 6, 8),   //  6
            Push(17, 7, 8),   //  7
            Push(19, 8, 8),   //  8
            Push(23, 9, 16),  //  9
            Push(27, 10, 16), // 10
            Push(29, 11, 16), // 11
            Push(31, 12, 16), // 12
            Push(37, 13, 16), // 13
            Push(41, 14, 16), // 14
            Push(43, 15, 16), // 15
            Push(47, 16, 16), // 16
            Push(53, 17, 32), // 17
            Pop(16, 32),      // 18
            Pop(15, 32),      // 19
            Pop(14, 32),      // 20
            Pop(13, 32),      // 21
            Pop(12, 32),      // 22
            Pop(11, 32),      // 23
            Pop(10, 32),      // 24
            Pop(9, 32),       // 25
            Pop(8, 32),       // 26
            Pop(7, 16),       // 27
            Pop(6, 16),       // 28
            Push(59, 7, 16),  // 29
            Push(61, 8, 16),  // 30
            Push(67, 9, 16),  // 31
            Push(71, 10, 16), // 32
            Pop(9, 16),       // 33
            Pop(8, 16),       // 34
            Pop(7, 16),       // 35
            Pop(6, 16),       // 36
            Pop(5, 16),       // 37
            Pop(4, 16),       // 38
            Pop(3, 8),        // 39
            Push(73, 4, 8),   // 40
            Pop(3, 8),        // 41
            Push(79, 4, 8),   // 42
            Push(83, 5, 8),   // 43
            Push(89, 6, 8),   // 44
            Push(97, 7, 8),   // 45
            Pop(6, 8),        // 46
            Pop(5, 8),        // 47
            Pop(4, 8),        // 48
            Pop(3, 8),        // 49
            Pop(2, 8),        // 50
            Pop(1, 4),        // 51
            Pop(0, 0),        // 52
        ] as &[_];

        for operation in operations {
            match *operation {
                Push(value, expected_len, expected_capacity) => {
                    table.push(value);
                    control.push(value);

                    assert_eq!(table.len(), expected_len);
                    assert_eq!(table.capacity(), expected_capacity);
                }
                Pop(expected_len, expected_capacity) => {
                    assert_eq!(table.pop(), control.pop());
                    assert_eq!(table.len(), expected_len);
                    assert_eq!(table.capacity(), expected_capacity);
                }
            }

            assert_eq!(table.is_empty(), table.len().eq(&0));
        }
    }
}
