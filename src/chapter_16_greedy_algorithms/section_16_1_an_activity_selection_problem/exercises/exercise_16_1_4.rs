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

pub fn schedule_activities(activities: &[(u64, u64)]) -> Box<[usize]> {
    let mut time_points = activities
        .iter()
        .enumerate()
        .flat_map(|(i, &(s, f))| iter::once((i, TimePoint::start(s))).chain(iter::once((i, TimePoint::finish(f)))))
        .collect::<Vec<_>>();

    time_points.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

    let mut free_slots = Vec::new();
    let mut max_slots = 0;
    let mut result = vec![usize::max_value(); activities.len()];

    for (i, TimePoint { is_start, .. }) in time_points {
        if is_start {
            result[i] = if let Some(free_slot) = free_slots.pop() {
                free_slot
            } else {
                let slot = max_slots;

                max_slots += 1;

                slot
            };
        } else {
            free_slots.push(result[i]);
        }
    }

    result.into()
}

#[cfg(test)]
pub mod tests {
    use super::schedule_activities;

    #[test]
    fn test_schedule_activities() {
        let activities = [
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
        ];

        let result = schedule_activities(&activities);

        assert_eq!(*result, [1, 3, 0, 3, 4, 1, 0, 3, 5, 2, 5]);
    }
}
