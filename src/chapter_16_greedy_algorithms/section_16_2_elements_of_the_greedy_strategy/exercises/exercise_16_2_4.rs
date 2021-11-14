#[must_use]
pub fn select_water_stops(water_stops: &[u64], total_distance: u64, max_skate_length: u64) -> Option<Box<[usize]>> {
    let mut result = Vec::new();
    let mut iter = water_stops.iter().copied().enumerate();
    let mut farest_distance = max_skate_length;

    if let Some((mut i, mut stop)) = iter.next() {
        'k: loop {
            // At the (i - 1)-th stop.

            if stop <= farest_distance {
                for (j, next_stop) in iter.by_ref() {
                    if next_stop <= farest_distance {
                        i = j;
                        stop = next_stop;
                    } else {
                        result.push(i);
                        farest_distance = stop + max_skate_length;
                        i = j;
                        stop = next_stop;

                        continue 'k;
                    }
                }

                result.push(i);
                farest_distance = stop + max_skate_length;

                break;
            }

            return None;
        }
    }

    (total_distance <= farest_distance).then(|| result.into())
}

#[cfg(test)]
mod tests {
    use super::select_water_stops;

    fn run_test(water_stops: &[u64], total_distance: u64, max_skate_length: u64, expected_result: Option<&[usize]>) {
        let result = select_water_stops(water_stops, total_distance, max_skate_length);

        assert_eq!(result.as_deref(), expected_result);
    }

    #[test]
    fn test_select_water_stops_empty_unreachable() {
        run_test(&[], 10, 5, None);
    }

    #[test]
    fn test_select_water_stops_empty_reachable() {
        run_test(&[], 10, 10, Some(&[]));
        run_test(&[], 10, 15, Some(&[]));
    }

    #[test]
    fn test_select_water_stops_first_water_stop_unreachable() {
        run_test(&[15], 20, 10, None);
    }

    #[test]
    fn test_select_water_stops_first_water_stop_reachable() {
        run_test(&[8], 15, 10, Some(&[0]));
        run_test(&[8], 20, 10, None);
    }

    #[test]
    fn test_select_water_stops_skip_first() {
        run_test(&[4, 8], 15, 10, Some(&[1]));
        run_test(&[4, 8, 16], 20, 10, Some(&[1, 2]));
    }

    #[test]
    fn test_select_water_stops_second_unreachable() {
        run_test(&[4, 8, 16], 30, 10, None);
    }
}
