// Assume the points are sorted in increasing order.

#[must_use]
pub fn cover_points(mut points: &[f64]) -> Box<[f64]> {
    let mut result = Vec::new();

    while let Some((&first_point, rest_points)) = points.split_first() {
        result.push(first_point);

        let interval_end = first_point + 1.0;

        if let Some(next) = rest_points.iter().position(|x| *x > interval_end) {
            points = &rest_points[next..];
        } else {
            break;
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    #[allow(clippy::manual_assert)]
    #[test]
    fn test_cover_points() {
        let points = [0.46, 2.08, 2.24, 3.27, 4.23, 4.58, 6.20, 7.63, 8.79, 9.62];
        let result = super::cover_points(&points);
        let expected_result = [0.46, 2.08, 3.27, 4.58, 6.20, 7.63, 8.79];

        approx::assert_ulps_eq!(*result, expected_result);
    }
}
