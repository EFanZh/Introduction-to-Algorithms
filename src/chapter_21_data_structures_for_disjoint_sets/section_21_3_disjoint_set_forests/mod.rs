use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

pub mod exercises;

pub struct Node<T> {
    pub value: T,
    parent: Cell<Option<Rc<Self>>>,
    rank: Cell<u8>,
}

pub fn make_set<T>(value: T) -> Rc<Node<T>> {
    Rc::new(Node {
        value,
        parent: Cell::new(None),
        rank: Cell::new(0),
    })
}

pub fn link<T>(x: &Rc<Node<T>>, y: &Rc<Node<T>>) {
    match x.rank.cmp(&y.rank) {
        Ordering::Less => x.parent.set(Some(y.clone())),
        Ordering::Equal => {
            x.parent.set(Some(y.clone()));
            y.rank.set(y.rank.get() + 1);
        }
        Ordering::Greater => y.parent.set(Some(x.clone())),
    }
}

pub fn find_set<T>(x: &Rc<Node<T>>) -> Rc<Node<T>> {
    x.parent.take().map_or_else(
        || x.clone(),
        |parent| {
            x.parent.set(Some(find_set(&parent)));

            parent
        },
    )
}

pub fn union<T>(x: &Rc<Node<T>>, y: &Rc<Node<T>>) {
    link(&find_set(x), &find_set(y));
}

#[cfg(test)]
mod tests {
    use super::{find_set, make_set, union};
    use std::collections::HashMap;

    enum Operation {
        MakeSet(i32),
        Union(i32, i32),
        FindSet(i32, i32),
    }

    #[test]
    fn test_make_set() {
        let result = make_set(1);

        assert_eq!(result.value, 1);
        assert!(result.parent.take().is_none());
        assert_eq!(result.rank.get(), 0);
    }

    #[test]
    fn test_union_find() {
        use Operation::{FindSet, MakeSet, Union};

        let test_cases = [
            &[
                MakeSet(1),
                MakeSet(2),
                FindSet(1, 1),
                FindSet(2, 2),
                Union(1, 2),
                FindSet(1, 2),
                FindSet(2, 2),
            ] as &[_],
            &[
                MakeSet(1),
                MakeSet(2),
                MakeSet(3),
                Union(1, 2),
                Union(2, 3),
                FindSet(1, 2),
                FindSet(2, 2),
                FindSet(3, 2),
            ],
            &[
                MakeSet(1),
                MakeSet(2),
                MakeSet(3),
                Union(1, 2),
                Union(3, 2),
                FindSet(1, 2),
                FindSet(2, 2),
                FindSet(3, 2),
            ],
        ];

        for operations in test_cases.iter().copied() {
            let mut environment = HashMap::new();

            for operation in operations {
                match operation {
                    MakeSet(value) => {
                        environment.insert(value, make_set(value));
                    }
                    Union(x, y) => union(&environment[x], &environment[y]),
                    FindSet(value, expected_root) => assert_eq!(find_set(&environment[value]).value, expected_root),
                }
            }
        }
    }
}
