use super::super::{schedule_task, Task};

pub fn solve() -> Box<[usize]> {
    let tasks = [(4, 70), (2, 60), (4, 50), (3, 40), (1, 30), (4, 20), (6, 10)];

    schedule_task(
        &tasks
            .iter()
            .copied()
            .map(|(d, w)| Task {
                deadline: d,
                penalty: (80 - w),
            })
            .collect::<Box<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_solve() {
        let result = solve();

        assert_eq!(*result, [4, 3, 5, 2, 6, 1, 0]);
    }
}
