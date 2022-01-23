use std::cmp::Reverse;

fn memoized_cut_rod_aux(p: &[u32], revenue_and_first_cuts: &mut [Option<(u32, usize)>]) -> u32 {
    let n = p.len();

    if let Some((revenue, _)) = revenue_and_first_cuts[n] {
        revenue
    } else {
        let (revenue, first_cut) = p
            .iter()
            .enumerate()
            .map(|(i, p_i)| {
                let length = i + 1;

                (
                    p_i + memoized_cut_rod_aux(&p[..n - length], revenue_and_first_cuts),
                    length,
                )
            })
            .max_by_key(|&(r, l)| (r, Reverse(l))) // Rust uses the last element when there are more than one maximums.
            .unwrap_or((0, 0));

        revenue_and_first_cuts[n] = Some((revenue, first_cut));

        revenue
    }
}

#[must_use]
pub fn memoized_cut_rod_with_solution(p: &[u32]) -> (u32, Box<[usize]>) {
    let n = p.len();
    let mut revenue_and_first_cuts = vec![None; n + 1];
    let result = memoized_cut_rod_aux(p, &mut revenue_and_first_cuts);

    let solution = {
        let mut solution = Vec::new();
        let mut length = n;

        while length > 0 {
            let first_cut = revenue_and_first_cuts[length].unwrap().1;

            solution.push(first_cut);
            length -= first_cut;
        }

        solution
    };

    (result, solution.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_memoized_cut_rod_with_solution() {
        assert_eq!(super::memoized_cut_rod_with_solution(&[]), (0, vec![].into()));
        assert_eq!(super::memoized_cut_rod_with_solution(&[1]), (1, vec![1].into()));
        assert_eq!(super::memoized_cut_rod_with_solution(&[1, 5]), (5, vec![2].into()));
        assert_eq!(super::memoized_cut_rod_with_solution(&[1, 5, 8]), (8, vec![3].into()));
        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9]),
            (10, vec![2, 2].into())
        );

        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10]),
            (13, vec![2, 3].into())
        );
        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10, 17]),
            (17, vec![6].into())
        );

        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10, 17, 17]),
            (18, vec![1, 6].into())
        );
        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10, 17, 17, 20]),
            (22, vec![2, 6].into())
        );

        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10, 17, 17, 20, 24]),
            (25, vec![3, 6].into())
        );

        assert_eq!(
            super::memoized_cut_rod_with_solution(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30]),
            (30, vec![10].into())
        );
    }
}
