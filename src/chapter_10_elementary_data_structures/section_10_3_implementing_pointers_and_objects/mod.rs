pub mod exercises;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Handle(usize);

const NIL: Handle = Handle(usize::max_value());

pub struct MultipleArrayDoublyLinkedListElementPool<T> {
    key: Vec<T>,
    next: Vec<Handle>,
    prev: Vec<Handle>,
    free: Handle,
}

impl<T> Default for MultipleArrayDoublyLinkedListElementPool<T> {
    fn default() -> Self {
        MultipleArrayDoublyLinkedListElementPool {
            key: Vec::new(),
            next: Vec::new(),
            prev: Vec::new(),
            free: NIL,
        }
    }
}

impl<T> MultipleArrayDoublyLinkedListElementPool<T> {
    pub fn new() -> MultipleArrayDoublyLinkedListElementPool<T> {
        Default::default()
    }

    // Allocate-Object()
    //
    // 1  if free == NIL
    // 2      error “out of space”
    // 3  else x = free
    // 4      free = x.next
    // 5      return x

    pub fn allocate_object(&mut self, value: T) -> Handle {
        if self.free == NIL {
            // We have more space!

            let x = self.key.len();

            self.key.push(value);
            self.next.push(NIL);
            self.prev.push(NIL);

            Handle(x)
        } else {
            let x = self.free;

            self.free = self.next[self.free.0];

            self.key[x.0] = value;
            self.next[x.0] = NIL;
            self.prev[x.0] = NIL;

            x
        }
    }

    // Free-Object(x)
    //
    // 1  x.next = free
    // 2  free = x

    pub fn free_object(&mut self, x: Handle) {
        self.next[x.0] = self.free;
        self.free = x;
    }

    pub fn get_key(&self, handle: Handle) -> &T {
        &self.key[handle.0]
    }

    pub fn get_next(&self, handle: Handle) -> Handle {
        self.next[handle.0]
    }

    pub fn get_prev(&self, handle: Handle) -> Handle {
        self.prev[handle.0]
    }
}

#[cfg(test)]
mod tests {
    use super::MultipleArrayDoublyLinkedListElementPool;

    #[test]
    fn test_multiple_array_doubly_linked_list_element_pool_allocate() {
        let mut pool = MultipleArrayDoublyLinkedListElementPool::new();
        let handle_1 = pool.allocate_object(42);

        assert_eq!(*pool.get_key(handle_1), 42);

        let handle_2 = pool.allocate_object(7);

        assert_eq!(*pool.get_key(handle_1), 42);
        assert_eq!(*pool.get_key(handle_2), 7);

        let handle_3 = pool.allocate_object(8);

        assert_eq!(*pool.get_key(handle_1), 42);
        assert_eq!(*pool.get_key(handle_2), 7);
        assert_eq!(*pool.get_key(handle_3), 8);
    }

    #[test]
    fn test_multiple_array_doubly_linked_list_element_pool_free() {
        let mut pool = MultipleArrayDoublyLinkedListElementPool::new();

        let handle_1 = pool.allocate_object(3);
        let handle_2 = pool.allocate_object(5);

        pool.free_object(handle_1);

        let handle_3 = pool.allocate_object(7);

        assert_eq!(*pool.get_key(handle_2), 5);
        assert_eq!(*pool.get_key(handle_3), 7);
    }
}
