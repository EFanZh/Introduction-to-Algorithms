use std::collections::VecDeque;
use std::iter;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct TimePoint {
    time: u64,
    is_start: bool,
}

impl TimePoint {
    fn start(time: u64) -> Self {
        Self { time, is_start: true }
    }

    fn finish(time: u64) -> Self {
        Self { time, is_start: false }
    }
}

#[must_use]
pub fn schedule_activities(activities: &[(u64, u64)]) -> Box<[usize]> {
    let mut time_points = activities
        .iter()
        .enumerate()
        .flat_map(|(i, &(s, f))| iter::once((i, TimePoint::start(s))).chain(iter::once((i, TimePoint::finish(f)))))
        .collect::<Vec<_>>();

    time_points.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

    let mut free_slots = VecDeque::new();
    let mut max_slots = 0;
    let mut result = vec![usize::MAX; activities.len()];

    for (i, TimePoint { is_start, .. }) in time_points {
        if is_start {
            result[i] = free_slots.pop_front().unwrap_or_else(|| {
                let slot = max_slots;

                max_slots += 1;

                slot
            });
        } else {
            free_slots.push_back(result[i]);
        }
    }

    result.into()
}

#[cfg(test)]
pub mod tests {
    use super::schedule_activities;

    #[test]
    fn test_schedule_activities() {
        type TestCase<'a> = (&'a [(u64, u64)], &'a [usize]);

        let test_cases: &[TestCase] = &[
            (
                &[
                    (1, 4),
                    (3, 5),
                    (0, 6),
                    (5, 7),
                    (3, 9),
                    (5, 9),
                    (6, 10),
                    (8, 11),
                    (8, 12),
                    (2, 14),
                    (12, 16),
                ],
                &[1, 3, 0, 1, 4, 3, 0, 1, 5, 2, 4],
            ),
            (&[(1, 4), (2, 5), (6, 7), (4, 8)], &[0, 1, 1, 0]),
        ];

        for (activities, expected_result) in test_cases.iter().copied() {
            assert_eq!(*schedule_activities(activities), *expected_result);
        }
    }
}
