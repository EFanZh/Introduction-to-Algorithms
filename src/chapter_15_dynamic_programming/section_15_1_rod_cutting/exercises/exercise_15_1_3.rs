pub fn extended_bottom_up_cut_rod_with_cutting_cost(p: &[u32], c: u32) -> (Box<[u32]>, Box<[usize]>) {
    let n = p.len();
    let mut revenues = vec![0; n + 1];
    let mut first_cuts = vec![0; n];

    for length in 1..=n {
        let mut max_revenue = p[length - 1];
        let mut best_first_cut = length;

        for first_cut in 1..length {
            let revenue = p[first_cut - 1] + revenues[length - first_cut] - c;

            if revenue > max_revenue {
                max_revenue = revenue;
                best_first_cut = first_cut
            }
        }

        first_cuts[length - 1] = best_first_cut;
        revenues[length] = max_revenue;
    }

    (revenues.into(), first_cuts.into())
}

#[cfg(test)]
mod tests {
    use super::extended_bottom_up_cut_rod_with_cutting_cost;

    #[test]
    fn test_extended_bottom_up_cut_rod_with_cutting_cost_1() {
        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[], 0),
            (vec![0].into(), vec![].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1], 0),
            (vec![0, 1].into(), vec![1].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5], 0),
            (vec![0, 1, 5].into(), vec![1, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8], 0),
            (vec![0, 1, 5, 8].into(), vec![1, 2, 3].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9], 0),
            (vec![0, 1, 5, 8, 10].into(), vec![1, 2, 3, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10], 0),
            (vec![0, 1, 5, 8, 10, 13].into(), vec![1, 2, 3, 2, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17], 0),
            (vec![0, 1, 5, 8, 10, 13, 17].into(), vec![1, 2, 3, 2, 2, 6].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17], 0),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18].into(),
                vec![1, 2, 3, 2, 2, 6, 1].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20], 0),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20, 24], 0),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22, 25].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2, 3].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30], 0),
            (
                vec![0, 1, 5, 8, 10, 13, 17, 18, 22, 25, 30].into(),
                vec![1, 2, 3, 2, 2, 6, 1, 2, 3, 10].into()
            )
        );
    }

    #[test]
    fn test_extended_bottom_up_cut_rod_with_cutting_cost_2() {
        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[], 2),
            (vec![0].into(), vec![].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1], 2),
            (vec![0, 1].into(), vec![1].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5], 2),
            (vec![0, 1, 5].into(), vec![1, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8], 2),
            (vec![0, 1, 5, 8].into(), vec![1, 2, 3].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9], 2),
            (vec![0, 1, 5, 8, 9].into(), vec![1, 2, 3, 4].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10], 2),
            (vec![0, 1, 5, 8, 9, 11].into(), vec![1, 2, 3, 4, 2].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17], 2),
            (vec![0, 1, 5, 8, 9, 11, 17].into(), vec![1, 2, 3, 4, 2, 6].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17], 2),
            (vec![0, 1, 5, 8, 9, 11, 17, 17].into(), vec![1, 2, 3, 4, 2, 6, 7].into())
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20], 2),
            (
                vec![0, 1, 5, 8, 9, 11, 17, 17, 20].into(),
                vec![1, 2, 3, 4, 2, 6, 7, 8].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20, 24], 2),
            (
                vec![0, 1, 5, 8, 9, 11, 17, 17, 20, 24].into(),
                vec![1, 2, 3, 4, 2, 6, 7, 8, 9].into()
            )
        );

        assert_eq!(
            extended_bottom_up_cut_rod_with_cutting_cost(&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30], 2),
            (
                vec![0, 1, 5, 8, 9, 11, 17, 17, 20, 24, 30].into(),
                vec![1, 2, 3, 4, 2, 6, 7, 8, 9, 10].into()
            )
        );
    }
}
