#[must_use]
pub fn schedule_tasks(tasks: &[u64]) -> Box<[usize]> {
    let mut result = tasks.iter().copied().enumerate().collect::<Vec<_>>();

    result.sort_by_key(|(_, key)| *key);

    result.into_iter().map(|(i, _)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::schedule_tasks;

    #[test]
    fn test_schedule_tasks() {
        assert_eq!(*schedule_tasks(&[]), []);
        assert_eq!(*schedule_tasks(&[7]), [0]);

        assert_eq!(*schedule_tasks(&[2, 3, 5]), [0, 1, 2]);
        assert_eq!(*schedule_tasks(&[3, 5, 2]), [2, 0, 1]);
        assert_eq!(*schedule_tasks(&[7, 6, 1]), [2, 1, 0]);
    }
}
