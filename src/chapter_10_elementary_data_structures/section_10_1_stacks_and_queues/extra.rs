pub trait Stack<T> {
    fn empty(&self) -> bool;
    fn push(&mut self, x: T);
    fn pop(&mut self) -> T;
    fn length(&self) -> usize;
}

pub trait Queue<T> {
    fn empty(&self) -> bool;
    fn enqueue(&mut self, x: T);
    fn dequeue(&mut self) -> T;
    fn length(&self) -> usize;
}
