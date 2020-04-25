pub fn is_valid_search_sequence<'a, T: Ord>(values: &'a [T], mut min_value: &'a T, mut max_value: &'a T) -> bool {
    if let Some((first, rest)) = values.split_first() {
        let mut previous = first;

        for value in rest {
            if value < min_value || value > max_value {
                return false;
            } else if value < previous {
                max_value = previous;
            } else {
                min_value = previous;
            }

            previous = value;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_search_sequence;

    fn is_valid_i32_search_sequence(values: &[i32]) -> bool {
        is_valid_search_sequence(values, &i32::MIN, &i32::MAX)
    }

    #[test]
    fn test_is_valid_search_sequence() {
        assert!(is_valid_i32_search_sequence(&[2, 252, 401, 398, 330, 344, 397, 363]));
        assert!(is_valid_i32_search_sequence(&[924, 220, 911, 244, 898, 258, 362, 363]));
        assert!(!is_valid_i32_search_sequence(&[925, 202, 911, 240, 912, 245, 363]));

        assert!(is_valid_i32_search_sequence(&[
            2, 399, 387, 219, 266, 382, 381, 278, 363
        ]));

        assert!(!is_valid_i32_search_sequence(&[935, 278, 347, 621, 299, 392, 358, 363]));
    }
}
