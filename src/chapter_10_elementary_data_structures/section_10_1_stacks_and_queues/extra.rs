pub trait Stack<T> {
    fn empty(&self) -> bool;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
}

pub trait Queue<T> {
    fn empty(&self) -> bool;
    fn enqueue(&mut self, x: T);
    fn dequeue(&mut self) -> T;
}
