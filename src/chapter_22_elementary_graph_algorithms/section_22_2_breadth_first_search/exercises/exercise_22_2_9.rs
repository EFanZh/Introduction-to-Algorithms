use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

fn find_path_helper(tree: &HashMap<usize, (Vec<usize>, Vec<usize>)>, node: usize, result: &mut Vec<usize>) {
    result.push(node);

    for &next in &tree[&node].0 {
        find_path_helper(tree, next, result);

        result.push(node);
    }

    for &next in &tree[&node].1 {
        result.push(next);
        result.push(node);
    }
}

#[must_use]
pub fn find_path(graph: &[Vec<usize>]) -> Vec<usize> {
    let mut tree = HashMap::new();
    let mut queue = VecDeque::new();
    let mut node = 0;

    tree.insert(0, (Vec::new(), Vec::new()));

    loop {
        for &next in &graph[node] {
            if let Entry::Vacant(entry) = tree.entry(next) {
                entry.insert((Vec::new(), Vec::new()));
                tree.get_mut(&node).unwrap().0.push(next);
                queue.push_back(next);
            } else if next > node {
                tree.get_mut(&node).unwrap().1.push(next);
            }
        }

        if let Some(next) = queue.pop_front() {
            node = next;
        } else {
            break;
        }
    }

    let mut result = Vec::new();

    find_path_helper(&tree, 0, &mut result);

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_path() {
        #[allow(trivial_casts)] // Expected.
        let test_cases = [(
            &[&[1_usize, 2] as &[usize], &[0, 3, 4], &[0, 3, 4], &[1, 2], &[1, 2]] as &[&[usize]],
            &[0, 1, 3, 1, 4, 1, 0, 2, 3, 2, 4, 2, 0],
        )];

        for (graph, expected) in test_cases {
            assert_eq!(
                super::find_path(&graph.iter().map(|edge| edge.to_vec()).collect::<Box<_>>()),
                expected
            );
        }
    }
}
