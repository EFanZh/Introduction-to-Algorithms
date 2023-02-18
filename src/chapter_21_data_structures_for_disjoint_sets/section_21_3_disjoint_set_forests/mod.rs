use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

pub mod exercises;

pub struct Node<T> {
    value: T,
    parent: Cell<Option<Rc<Self>>>,
    rank: Cell<u8>,
}

impl<T> Node<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
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
        Ordering::Less => x.parent.set(Some(Rc::clone(y))),
        Ordering::Equal => {
            x.parent.set(Some(Rc::clone(y)));
            y.rank.set(y.rank.get() + 1);
        }
        Ordering::Greater => y.parent.set(Some(Rc::clone(x))),
    }
}

pub fn find_set<T>(x: &Rc<Node<T>>) -> Rc<Node<T>> {
    x.parent.take().map_or_else(
        || Rc::clone(x),
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
    use std::collections::HashMap;

    enum Operation {
        MakeSet(i32),
        Union(i32, i32),
        FindSet(i32, i32),
    }

    #[test]
    fn test_make_set() {
        let result = super::make_set(1);

        assert_eq!(result.value, 1);
        assert!(result.parent.take().is_none());
        assert_eq!(result.rank.get(), 0);
    }

    #[test]
    fn test_union_find() {
        #[allow(trivial_casts)] // Expected.
        let test_cases = [
            &[
                Operation::MakeSet(1),
                Operation::MakeSet(2),
                Operation::FindSet(1, 1),
                Operation::FindSet(2, 2),
                Operation::Union(1, 2),
                Operation::FindSet(1, 2),
                Operation::FindSet(2, 2),
            ] as &[_],
            &[
                Operation::MakeSet(1),
                Operation::MakeSet(2),
                Operation::MakeSet(3),
                Operation::Union(1, 2),
                Operation::Union(2, 3),
                Operation::FindSet(1, 2),
                Operation::FindSet(2, 2),
                Operation::FindSet(3, 2),
            ],
            &[
                Operation::MakeSet(1),
                Operation::MakeSet(2),
                Operation::MakeSet(3),
                Operation::Union(1, 2),
                Operation::Union(3, 2),
                Operation::FindSet(1, 2),
                Operation::FindSet(2, 2),
                Operation::FindSet(3, 2),
            ],
        ];

        for operations in test_cases {
            let mut environment = HashMap::new();

            for operation in operations {
                match operation {
                    Operation::MakeSet(value) => {
                        environment.insert(value, super::make_set(value));
                    }
                    Operation::Union(x, y) => super::union(&environment[x], &environment[y]),
                    Operation::FindSet(value, expected_root) => {
                        assert_eq!(super::find_set(&environment[value]).value, expected_root);
                    }
                }
            }
        }
    }
}
