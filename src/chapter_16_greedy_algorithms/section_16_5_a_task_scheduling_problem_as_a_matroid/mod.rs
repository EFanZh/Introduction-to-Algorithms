use exercises::exercise_16_5_2;
use std::cmp::Reverse;

pub mod exercises;

pub struct Task {
    pub deadline: usize,
    pub penalty: u64,
}

#[must_use]
pub fn schedule_task(tasks: &[Task]) -> Box<[usize]> {
    let mut early_tasks = Vec::new();
    let mut deadline_tasks = vec![0; tasks.len()];
    let mut late_tasks = Vec::new();
    let mut task_indices = (0..tasks.len()).collect::<Box<_>>();

    task_indices.sort_by_key(|i| Reverse(tasks[*i].penalty));

    for i in task_indices.iter().copied() {
        let task = &tasks[i];

        if exercise_16_5_2::check_schedule(&deadline_tasks, task.deadline) {
            early_tasks.push(i);
            deadline_tasks[task.deadline - 1] += 1;
        } else {
            late_tasks.push(i);
        }
    }

    early_tasks.sort_by_key(|i| tasks[*i].deadline);
    early_tasks.extend(late_tasks);

    early_tasks.into()
}

#[cfg(test)]
mod tests {
    use super::{schedule_task, Task};

    #[test]
    fn test_schedule_task() {
        let tasks = [(4, 70), (2, 60), (4, 50), (3, 40), (1, 30), (4, 20), (6, 10)];

        let result = schedule_task(
            &tasks
                .iter()
                .copied()
                .map(|(d, w)| Task {
                    deadline: d,
                    penalty: w,
                })
                .collect::<Box<_>>(),
        );

        assert_eq!(*result, [1, 3, 0, 2, 6, 4, 5]);
    }
}
