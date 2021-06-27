#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Handle(usize);

enum Slot<T> {
    Free(Handle),
    Occupied(T),
}

const NIL: Handle = Handle(usize::MAX);

pub struct ObjectPool<T> {
    memory: Vec<Slot<T>>,
    free: Handle,
}

impl<T> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ObjectPool<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            free: NIL,
        }
    }

    pub fn allocate_object(&mut self, value: T) -> Handle {
        if self.free == NIL {
            // We have more space!

            let x = self.memory.len();

            self.memory.push(Slot::Occupied(value));

            Handle(x)
        } else {
            let x = self.free;

            if let Slot::Free(next_handle) = self.memory[self.free.0] {
                self.free = next_handle;
            } else {
                panic!();
            }

            self.memory[x.0] = Slot::Occupied(value);

            x
        }
    }

    pub fn free_object(&mut self, x: Handle) {
        self.memory[x.0] = Slot::Free(self.free);
        self.free = x;
    }

    #[must_use]
    pub fn get(&self, handle: Handle) -> &T {
        if let Slot::Occupied(value) = &self.memory[handle.0] {
            value
        } else {
            panic!("Invalid handle");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectPool;

    #[test]
    fn test_object_pool_allocate() {
        let mut pool = ObjectPool::new();
        let handle_1 = pool.allocate_object(42);

        assert_eq!(*pool.get(handle_1), 42);

        let handle_2 = pool.allocate_object(7);

        assert_eq!(*pool.get(handle_1), 42);
        assert_eq!(*pool.get(handle_2), 7);

        let handle_3 = pool.allocate_object(8);

        assert_eq!(*pool.get(handle_1), 42);
        assert_eq!(*pool.get(handle_2), 7);
        assert_eq!(*pool.get(handle_3), 8);
    }

    #[test]
    fn test_object_pool_free() {
        let mut pool = ObjectPool::new();

        let handle_1 = pool.allocate_object(3);
        let handle_2 = pool.allocate_object(5);

        pool.free_object(handle_1);

        let handle_3 = pool.allocate_object(7);

        assert_eq!(*pool.get(handle_2), 5);
        assert_eq!(*pool.get(handle_3), 7);
    }
}
