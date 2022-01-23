fn construct_optimal_bst_helper<F: FnMut(usize, usize) -> usize>(
    root: &mut F,
    p: usize,
    i: usize,
    j: usize,
    position: &str,
    result: &mut Vec<String>,
) {
    if i == j {
        result.push(format!("d_{} is the {} child of k_{}", i, position, p));
    } else {
        let r = root(i, j);

        result.push(format!("k_{} is the {} child of k_{}", r, position, p));

        construct_optimal_bst_helper(root, r, i, r, "left", result);
        construct_optimal_bst_helper(root, r, r + 1, j, "right", result);
    }
}

#[must_use]
pub fn construct_optimal_bst(root: &[usize], n: usize) -> Box<[String]> {
    let mut result = Vec::new();
    let mut root_fn = |i, j| root[n * i + (j - 1)];

    if !root.is_empty() {
        let r = root_fn(0, n);

        result.push(format!("k_{} is the root", r));

        construct_optimal_bst_helper(&mut root_fn, r, 0, r, "left", &mut result);
        construct_optimal_bst_helper(&mut root_fn, r, r + 1, n, "right", &mut result);
    }

    result.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_construct_optimal_bst() {
        let root = [
            0, 0, 1, 1, 1, //
            0, 1, 1, 1, 3, //
            0, 0, 2, 3, 4, //
            0, 0, 0, 3, 4, //
            0, 0, 0, 0, 4, //
        ];

        assert_eq!(
            *super::construct_optimal_bst(&root, 5),
            [
                "k_1 is the root",
                "k_0 is the left child of k_1",
                "d_0 is the left child of k_0",
                "d_1 is the right child of k_0",
                "k_4 is the right child of k_1",
                "k_3 is the left child of k_4",
                "k_2 is the left child of k_3",
                "d_2 is the left child of k_2",
                "d_3 is the right child of k_2",
                "d_4 is the right child of k_3",
                "d_5 is the right child of k_4",
            ]
        );
    }
}
