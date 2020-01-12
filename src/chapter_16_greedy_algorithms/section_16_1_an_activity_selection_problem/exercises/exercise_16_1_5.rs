use std::iter;

pub struct Activity {
    pub start_time: u64,
    pub finish_time: u64,
    pub value: u64,
}

pub fn select_activities(activities: &[Activity]) -> (u64, Box<[usize]>) {
    let mut cache = vec![(0, 0); activities.len()];

    for (i, activity_i) in activities.iter().enumerate().rev() {
        let mut choice = cache
            .iter()
            .enumerate()
            .skip(i + 1)
            .filter_map(|(j, (v, _))| {
                if activities[j].start_time < activity_i.finish_time {
                    None
                } else {
                    Some((*v, j))
                }
            })
            .max_by_key(|(v, _)| *v)
            .unwrap_or((0, usize::max_value()));

        choice.0 += activity_i.value;

        cache[i] = choice;
    }

    let selected_activities = iter::successors(
        cache.iter().enumerate().max_by_key(|(_, (v, _))| *v).map(|(i, _)| i),
        |i| {
            let next = cache[*i].1;

            if next == usize::max_value() {
                None
            } else {
                Some(next)
            }
        },
    )
    .collect::<Box<_>>();

    let total_value = selected_activities.first().map(|i| cache[*i].0).unwrap_or(0);

    (total_value, selected_activities)
}

#[cfg(test)]
pub mod tests {
    use super::{select_activities, Activity};

    #[test]
    fn test_select_activities() {
        let activity_data = [
            (1, 4, 1),
            (3, 5, 3),
            (0, 6, 8),
            (5, 7, 2),
            (3, 9, 6),
            (5, 9, 5),
            (6, 10, 1),
            (8, 11, 4),
            (8, 12, 6),
            (2, 14, 2),
            (12, 16, 2),
        ];

        let activities = activity_data
            .iter()
            .map(|&(start_time, finish_time, value)| Activity {
                start_time,
                finish_time,
                value,
            })
            .collect::<Box<_>>();

        let (total_value, result) = select_activities(&activities);

        assert_eq!(total_value, 16);
        assert_eq!(*result, [2, 8, 10]);
    }
}
