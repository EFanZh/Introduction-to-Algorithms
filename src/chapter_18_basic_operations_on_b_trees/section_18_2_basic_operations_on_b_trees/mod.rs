use super::section_18_1_definition_of_b_trees::Node;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::mem;

pub mod exercises;

// B-Tree-Search(x, k)
//
// 1  i = 1
// 2  while i ≤ x.n and k > x.key_i
// 3      i = i + 1
// 4  if i ≤ x.n and k == x.key_i
// 5      return (x, i)
// 6  elseif x.leaf
// 7      return nil
// 8  else Disk-Read(x.c_i)
// 9      return B-Tree-Search(x.c_i, k)

pub fn b_tree_search<'a, K: Ord + Borrow<Q>, V, Q: Ord + ?Sized>(x: &'a Node<K, V>, k: &Q) -> Option<&'a V> {
    match x.data.binary_search_by(|(key, _)| key.borrow().cmp(k)) {
        Ok(i) => Some(&x.data[i].1),
        Err(i) => x.children.get(i).and_then(|child| b_tree_search(child, k)),
    }
}

// B-Tree-Create(T)
//
// 1 x = Allocate-Node()
// 2 x.leaf = true
// 3 x.n = 0
// 4 Disk-Write(x)
// 5 T.root = x

pub fn b_tree_create<K, V>() -> Node<K, V> {
    Node::leaf(Vec::new())
}

fn split_node<K, V>(node: &mut Node<K, V>) -> (K, V, Node<K, V>) {
    let half = node.data.len() / 2;
    let data = node.data.split_off(half + 1);
    let (key, value) = node.data.pop().unwrap();

    let z = Node {
        data,
        children: node.children.split_off((node.children.len() + 1) / 2),
    };

    (key, value, z)
}

// B-Tree-Split-Child(x, i)
//
//  1  z = Allocate-Node()
//  2  y = x.c_i
//  3  z.leaf = y.leaf
//  4  z.n = t - 1
//  5  for j = 1 to t - 1
//  6      z.key_j = y.key_{j + t}
//  7  if not y.leaf
//  8      for j = 1 to t
//  9          z.c_j = y.c_{j + t}
// 10  y.n = t - 1
// 11  for j = x.n + 1 downto i + 1
// 12      x.c_{j + 1} = x.c_j
// 13  x.c_{i + 1} = z
// 14  for j = x.n downto i
// 15      x.key_{j + 1} = x.key_j
// 16  x.key_i = y.key_t
// 17  x.n = x.n + 1
// 18  Disk-Write(y)
// 19  Disk-Write(z)
// 20  Disk-Write(x)

fn b_tree_split_child<K, V>(x: &mut Node<K, V>, i: usize) {
    let y = &mut x.children[i];

    let (key, value, z) = split_node(y);

    x.data.insert(i, (key, value));
    x.children.insert(i + 1, z);
}

// B-Tree-Insert-Nonfull(x, k)
//
//  1  i = x.n
//  2  if x.leaf
//  3      while i ≥ 1 and k < x.key_i
//  4          x.key_{i + 1} = x.key_i
//  5          i = i - 1
//  6      x.key_{i + 1} = k
//  7      x.n = x.n + 1
//  8      Disk-Write(x)
//  9  else while i ≥ 1 and k < x.key_i
// 10           i = i - 1
// 11       i = i + 1
// 12       Disk-Read(x.c_i)
// 13       if x.c_i.n == 2 t - 1
// 14       B-Tree-Split-Child(x, i)
// 15       if k > x.key_i
// 16           i = i + 1
// 17       B-Tree-Insert-Nonfull(x.c_i, k)

fn b_tree_insert_nonfull<K: Ord, V>(x: &mut Node<K, V>, t: usize, k: K, v: V) -> Option<V> {
    match x.data.binary_search_by(|(key, _)| key.cmp(&k)) {
        Ok(i) => Some(mem::replace(&mut x.data[i].1, v)),
        Err(i) => {
            if let Some(child) = x.children.get_mut(i) {
                let child = if child.data.len() == t * 2 - 1 {
                    b_tree_split_child(x, i);

                    let (new_key, new_value) = &mut x.data[i];

                    match k.cmp(new_key) {
                        Ordering::Less => &mut x.children[i],
                        Ordering::Equal => return Some(mem::replace(new_value, v)),
                        Ordering::Greater => &mut x.children[i + 1],
                    }
                } else {
                    child
                };

                b_tree_insert_nonfull(child, t, k, v)
            } else {
                x.data.insert(i, (k, v));

                None
            }
        }
    }
}

// B-Tree-Insert(T, k)
//
//  1  r = T.root
//  2  if r.n == 2 t - 1
//  3      s = Allocate-Node()
//  4      T.root = s
//  5      s.leaf = false
//  6      s.n = 0
//  7      s.c_1 = r
//  8      B-Tree-Split-Child(s, 1)
//  9      B-Tree-Insert-Nonfull(s, k)
// 10  else B-Tree-Insert-Nonfull(r, k)

pub fn b_tree_insert<K: Ord, V>(root: &mut Node<K, V>, t: usize, k: K, v: V) -> Option<V> {
    if root.data.len() == t * 2 - 1 {
        let r = mem::replace(
            root,
            Node {
                data: Vec::new(),
                children: Vec::with_capacity(1),
            },
        );

        root.children.push(r);

        b_tree_split_child(root, 0);
    }

    b_tree_insert_nonfull(root, t, k, v)
}

#[cfg(test)]
mod tests {
    use super::super::section_18_1_definition_of_b_trees::tests::make_node;
    use super::{b_tree_create, b_tree_insert, b_tree_search, b_tree_split_child};

    #[test]
    fn test_b_tree_search() {
        let test_cases = [
            ((make_node!(), 2), None),
            ((make_node!(1 => 3), 2), None),
            ((make_node!(2 => 3), 2), Some(3)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 1), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 2), Some(3)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 3), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 5), Some(7)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 6), None),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 11), Some(13)),
            ((make_node!(2 => 3, 5 => 7, 11 => 13), 12), None),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 1),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 2),
                Some(3),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 3),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 5),
                Some(7),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 6),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 11),
                Some(13),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 12),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 17),
                Some(19),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 18),
                None,
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 23),
                Some(27),
            ),
            (
                (make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 27)), 24),
                None,
            ),
        ];

        for ((tree, key), expected) in test_cases.iter().cloned() {
            assert_eq!(b_tree_search(&tree, &key).copied(), expected)
        }
    }

    #[test]
    fn test_b_tree_create() {
        assert_eq!(b_tree_create::<i32, i32>(), make_node!());
    }

    #[test]
    fn test_b_tree_split_child() {
        let test_cases = [
            (
                (
                    make_node!((2 => 3, 5 => 7, 11 => 13), 17 => 19, (23 => 29, 31 => 37, 41 => 43)),
                    0,
                ),
                make_node!((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29, 31 => 37, 41 => 43)),
            ),
            (
                (
                    make_node!((2 => 3, 5 => 7, 11 => 13), 17 => 19, (23 => 29, 31 => 37, 41 => 43)),
                    1,
                ),
                make_node!((2 => 3, 5 => 7, 11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79), 83 => 89, (97 => 101), 103 => 107, (109 => 113))
                    ),
                    0,
                ),
                make_node!(
                    ((2 => 3), 5 => 7, (11 => 13)),
                    17 => 19,
                    ((23 => 29), 31 => 37, (41 => 43)),
                    47 => 53,
                    ((59 => 61), 67 => 71, (73 => 79), 83 => 89, (97 => 101), 103 => 107, (109 => 113))
                ),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79), 83 => 89, (97 => 101), 103 => 107, (109 => 113))
                    ),
                    1,
                ),
                make_node!(
                    ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                    47 => 53,
                    ((59 => 61), 67 => 71, (73 => 79)),
                    83 => 89,
                    ((97 => 101), 103 => 107, (109 => 113))
                ),
            ),
        ];

        for ((mut x, i), expected) in test_cases.iter().cloned() {
            b_tree_split_child(&mut x, i);

            assert_eq!(x, expected);
        }
    }

    #[test]
    fn test_b_tree_insert_1() {
        let test_cases = [
            ((make_node!(), 2, 2, 3), (None, make_node!(2 => 3))),
            ((make_node!(2 => 3), 2, 1, 4), (None, make_node!(1 => 4, 2 => 3))),
            ((make_node!(2 => 3), 2, 2, 4), (Some(3), make_node!(2 => 4))),
            ((make_node!(2 => 3), 2, 3, 4), (None, make_node!(2 => 3, 3 => 4))),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 1, 4),
                (None, make_node!((1 => 4, 2 => 3), 5 => 7, (11 => 13))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 2, 4),
                (Some(3), make_node!((2 => 4), 5 => 7, (11 => 13))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 3, 4),
                (None, make_node!((2 => 3, 3 => 4), 5 => 7, (11 => 13))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 5, 4),
                (Some(7), make_node!((2 => 3), 5 => 4, (11 => 13))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 6, 4),
                (None, make_node!((2 => 3), 5 => 7, (6 => 4, 11 => 13))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 11, 4),
                (Some(13), make_node!((2 => 3), 5 => 7, (11 => 4))),
            ),
            (
                (make_node!(2 => 3, 5 => 7, 11 => 13), 2, 12, 4),
                (None, make_node!((2 => 3), 5 => 7, (11 => 13, 12 => 4))),
            ),
        ];

        for ((mut root, t, key, value), (expected_result, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(b_tree_insert(&mut root, t, key, value), expected_result);
            assert_eq!(root, expected_tree);
        }
    }

    #[test]
    fn test_b_tree_insert_2() {
        let test_cases = [
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                    2,
                    1,
                    4,
                ),
                (
                    None,
                    make_node!(
                        ((1 => 4, 2 => 3), 5 => 7, (11 => 13)),
                        17 => 19,
                        ((23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                ),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                    2,
                    17,
                    4,
                ),
                (
                    Some(19),
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13)),
                        17 => 4,
                        ((23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                ),
            ),
            (
                (
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13), 17 => 19, (23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                    2,
                    18,
                    4,
                ),
                (
                    None,
                    make_node!(
                        ((2 => 3), 5 => 7, (11 => 13)),
                        17 => 19,
                        ((18 => 4, 23 => 29), 31 => 37, (41 => 43)),
                        47 => 53,
                        ((59 => 61), 67 => 71, (73 => 79))
                    ),
                ),
            ),
        ];

        for ((mut root, t, key, value), (expected_result, expected_tree)) in test_cases.iter().cloned() {
            assert_eq!(b_tree_insert(&mut root, t, key, value), expected_result);
            assert_eq!(root, expected_tree);
        }
    }
}
