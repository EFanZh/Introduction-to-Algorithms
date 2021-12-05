use crate::utilities::KeyValuePair;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Task {
    pub release_time: u64,
    pub processing_time: u64,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ScheduleItemType {
    Idle,
    Work(usize),
}

#[derive(PartialEq, Eq, Debug)]
pub struct ScheduleItem {
    pub schedule_type: ScheduleItemType,
    pub duration: u64,
}

impl ScheduleItem {
    fn idle(duration: u64) -> Self {
        Self {
            schedule_type: ScheduleItemType::Idle,
            duration,
        }
    }

    fn work(task_id: usize, duration: u64) -> Self {
        Self {
            schedule_type: ScheduleItemType::Work(task_id),
            duration,
        }
    }
}

struct ScheduleBuilder {
    result: Vec<ScheduleItem>,
}

impl ScheduleBuilder {
    fn add(&mut self, item: ScheduleItem) {
        if let Some(last_item) = self.result.last_mut() {
            if last_item.schedule_type == item.schedule_type {
                last_item.duration += item.duration;
            } else {
                self.result.push(item);
            }
        } else {
            self.result.push(item);
        }
    }

    fn build(self) -> Box<[ScheduleItem]> {
        self.result.into()
    }
}

type QueueItem = KeyValuePair<Reverse<u64>, usize>;

fn make_queue_item(task_id: usize, remaining_time: u64) -> QueueItem {
    KeyValuePair::new(Reverse(remaining_time), task_id)
}

#[must_use]
pub fn schedule_tasks(tasks: &[Task]) -> Box<[ScheduleItem]> {
    let mut tasks_by_release_time = tasks.iter().enumerate().collect::<Vec<_>>();

    tasks_by_release_time.sort_by_key(|(_, task)| task.release_time);

    let mut new_task_iter = tasks_by_release_time.into_iter().peekable();
    let mut time = 0;
    let mut queue = BinaryHeap::new();
    let mut schedule_builder = ScheduleBuilder { result: Vec::new() };

    loop {
        if let Some(KeyValuePair {
            key: Reverse(remaining_time),
            value: task_id,
        }) = queue.pop()
        {
            if let Some((next_task_id, next_task)) = new_task_iter.peek() {
                let finish_time = time + remaining_time;

                if finish_time <= next_task.release_time {
                    schedule_builder.add(ScheduleItem::work(task_id, remaining_time));
                    time = finish_time;
                } else {
                    let time_to_next_release = next_task.release_time - time;

                    schedule_builder.add(ScheduleItem::work(task_id, time_to_next_release));

                    queue.push(make_queue_item(task_id, remaining_time - time_to_next_release));
                    queue.push(make_queue_item(*next_task_id, next_task.processing_time));

                    time = next_task.release_time;

                    new_task_iter.next();
                }
            } else {
                schedule_builder.add(ScheduleItem::work(task_id, remaining_time));
                time += remaining_time;
            }
        } else if let Some((next_task_id, next_task)) = new_task_iter.next() {
            if next_task.release_time > time {
                schedule_builder.add(ScheduleItem::idle(next_task.release_time - time));
                time = next_task.release_time;
            }

            queue.push(make_queue_item(next_task_id, next_task.processing_time));
        } else {
            break;
        }
    }

    schedule_builder.build()
}

#[cfg(test)]
mod tests {
    use super::{schedule_tasks, ScheduleItem, Task};

    #[test]
    fn test_schedule_tasks() {
        fn run_test(tasks: &[(u64, u64)], expected: &[ScheduleItem]) {
            let tasks = tasks
                .iter()
                .map(|&(release_time, processing_time)| Task {
                    release_time,
                    processing_time,
                })
                .collect::<Box<_>>();

            let result = schedule_tasks(&tasks);

            assert_eq!(*result, *expected);
        }

        let idle = ScheduleItem::idle;
        let work = ScheduleItem::work;

        run_test(&[], &[]);
        run_test(&[(0, 4)], &[work(0, 4)]);
        run_test(&[(1, 4)], &[idle(1), work(0, 4)]);

        run_test(
            &[(1, 6), (4, 6), (11, 2)],
            &[idle(1), work(0, 6), work(1, 6), work(2, 2)],
        );

        run_test(&[(2, 8), (3, 1)], &[idle(2), work(0, 1), work(1, 1), work(0, 7)]);

        run_test(&[(2, 1), (4, 1)], &[idle(2), work(0, 1), idle(1), work(1, 1)]);
    }
}
