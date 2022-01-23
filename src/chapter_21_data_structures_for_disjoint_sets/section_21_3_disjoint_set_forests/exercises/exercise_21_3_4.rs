use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

pub struct Node<T> {
    pub value: T,
    parent: Cell<Option<Rc<Self>>>,
    rank: Cell<u8>,
    next: Cell<Weak<Self>>,
}

pub fn make_set<T>(value: T) -> Rc<Node<T>> {
    let result = Rc::new(Node {
        value,
        parent: Cell::new(None),
        rank: Cell::new(0),
        next: Cell::new(Weak::new()),
    });

    result.next.set(Rc::downgrade(&result));

    result
}

fn use_cell<T: Default, U>(cell: &Cell<T>, f: impl FnOnce(&T) -> U) -> U {
    let value = cell.take();
    let result = f(&value);

    cell.set(value);

    result
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

    let x_next = use_cell(&x.next, Clone::clone);
    let y_next = use_cell(&y.next, Clone::clone);

    x.next.set(y_next);
    y.next.set(x_next);
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

pub fn print_set<T>(x: &Rc<Node<T>>, mut f: impl FnMut(&T)) {
    f(&x.value);

    let mut node = use_cell(&x.next, Weak::upgrade).unwrap();

    while !Rc::ptr_eq(&node, x) {
        f(&node.value);

        node = use_cell(&node.next, Weak::upgrade).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

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
        use Operation::{FindSet, MakeSet, Union};

        #[allow(trivial_casts)]
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

        for operations in test_cases {
            let mut environment = HashMap::new();

            for operation in operations {
                match operation {
                    MakeSet(value) => {
                        environment.insert(value, super::make_set(value));
                    }
                    Union(x, y) => super::union(&environment[x], &environment[y]),
                    FindSet(value, expected_root) => {
                        assert_eq!(super::find_set(&environment[value]).value, expected_root);
                    }
                }
            }
        }
    }

    #[test]
    fn test_print_set() {
        fn collect_set(x: &Rc<Node<i32>>) -> HashSet<i32> {
            let mut result = HashSet::new();

            super::print_set(x, |value| {
                result.insert(*value);
            });

            result
        }

        fn array_to_hash_set(values: &[i32]) -> HashSet<i32> {
            values.iter().copied().collect()
        }

        let x = super::make_set(2);

        assert_eq!(collect_set(&x), array_to_hash_set(&[2]));

        let y = super::make_set(3);

        assert_eq!(collect_set(&y), array_to_hash_set(&[3]));

        super::union(&x, &y);

        assert_eq!(collect_set(&x), array_to_hash_set(&[2, 3]));
        assert_eq!(collect_set(&y), array_to_hash_set(&[2, 3]));

        let z = super::make_set(5);

        super::union(&x, &z);

        assert_eq!(collect_set(&x), array_to_hash_set(&[2, 3, 5]));
        assert_eq!(collect_set(&y), array_to_hash_set(&[2, 3, 5]));
        assert_eq!(collect_set(&z), array_to_hash_set(&[2, 3, 5]));
    }
}
