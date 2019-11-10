use super::super::{matrix_chain_order, print_optimal_parens};

pub fn solve() -> String {
    let p = [5, 10, 3, 12, 5, 50, 6];
    let (_, s) = matrix_chain_order(&p);

    print_optimal_parens(&s, 0, p.len() - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), "((A_0 A_1) ((A_2 A_3) (A_4 A_5)))");
    }
}
