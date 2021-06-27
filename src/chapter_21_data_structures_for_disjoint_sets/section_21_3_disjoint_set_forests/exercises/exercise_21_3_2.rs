use super::super::Node;
use std::rc::Rc;

pub fn find_set<T>(x: &Rc<Node<T>>) -> Rc<Node<T>> {
    let mut root = Rc::clone(x);

    while let Some(parent) = root.parent.take() {
        root.parent.set(Some(Rc::clone(&parent)));

        root = parent;
    }

    let mut node = Rc::clone(x);

    while let Some(parent) = node.parent.take() {
        node.parent.set(Some(Rc::clone(&root)));
        node = parent;
    }

    root
}

#[cfg(test)]
mod tests {
    use super::super::super::Node;
    use super::find_set;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_find_set_1() {
        let node = Rc::new(Node {
            value: 2,
            rank: Cell::new(0),
            parent: Cell::new(None),
        });

        assert_eq!(find_set(&node).value, 2);
        assert!(node.parent.take().is_none());
    }

    #[test]
    fn test_find_set_2() {
        let node_0 = Rc::new(Node {
            value: 2,
            rank: Cell::new(0),
            parent: Cell::new(None),
        });

        let node_1 = Rc::new(Node {
            value: 3,
            rank: Cell::new(1),
            parent: Cell::new(None),
        });

        node_0.parent.set(Some(Rc::clone(&node_1)));

        assert_eq!(find_set(&node_0).value, 3);
        assert_eq!(node_0.parent.take().unwrap().value, 3);
        assert!(node_1.parent.take().is_none());
    }

    #[test]
    fn test_find_set_3() {
        let node_0 = Rc::new(Node {
            value: 2,
            rank: Cell::new(0),
            parent: Cell::new(None),
        });

        let node_1 = Rc::new(Node {
            value: 3,
            rank: Cell::new(1),
            parent: Cell::new(None),
        });

        let node_2 = Rc::new(Node {
            value: 5,
            rank: Cell::new(2),
            parent: Cell::new(None),
        });

        node_0.parent.set(Some(Rc::clone(&node_1)));
        node_1.parent.set(Some(Rc::clone(&node_2)));

        assert_eq!(find_set(&node_0).value, 5);
        assert_eq!(node_0.parent.take().unwrap().value, 5);
        assert_eq!(node_1.parent.take().unwrap().value, 5);
        assert!(node_2.parent.take().is_none());
    }
}
