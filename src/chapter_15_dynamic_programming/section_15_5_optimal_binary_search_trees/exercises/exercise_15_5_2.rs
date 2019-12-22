use super::super::optimal_bst;
use super::exercise_15_5_1::construct_optimal_bst;

pub fn solve() -> (f64, Box<[String]>) {
    let p = [0.04, 0.06, 0.08, 0.02, 0.10, 0.12, 0.14];
    let q = [0.06, 0.06, 0.06, 0.06, 0.05, 0.05, 0.05, 0.05];
    let (e, root) = optimal_bst(&p, &q);

    (e[p.len()], construct_optimal_bst(&root, p.len()))
}

#[cfg(test)]
mod tests {
    use super::solve;
    use approx::assert_relative_eq;

    #[test]
    fn test_solve() {
        let (cost, construction) = solve();

        assert_relative_eq!(cost, 3.12);

        let expected_construction = [
            "k_4 is the root",
            "k_1 is the left child of k_4",
            "k_0 is the left child of k_1",
            "d_0 is the left child of k_0",
            "d_1 is the right child of k_0",
            "k_2 is the right child of k_1",
            "d_2 is the left child of k_2",
            "k_3 is the right child of k_2",
            "d_3 is the left child of k_3",
            "d_4 is the right child of k_3",
            "k_6 is the right child of k_4",
            "k_5 is the left child of k_6",
            "d_5 is the left child of k_5",
            "d_6 is the right child of k_5",
            "d_7 is the right child of k_6",
        ];

        assert_eq!(*construction, expected_construction);
    }
}
