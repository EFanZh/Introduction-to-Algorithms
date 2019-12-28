use std::iter;

#[derive(Clone, Copy, Debug)]
enum NextPixelLocation {
    None,
    Left,
    Middle,
    Right,
}

pub fn seam_carving(disruptions: &[f64], columns: usize) -> Box<[usize]> {
    let pixels = disruptions.len();
    let rows = pixels / columns;
    let index = |row: usize, column: usize| columns * row + column;
    let mut cache = vec![(0.0, NextPixelLocation::None); pixels];

    // Initialize last row.

    for (cache_item, disruption) in cache.iter_mut().zip(disruptions).skip(pixels - columns) {
        cache_item.0 = *disruption;
    }

    // Build cache.

    for row in (0..rows - 1).rev() {
        for column in (0..columns).rev() {
            // TDOD: Optimize with sliding window minimum algorithm?

            let mut choice = (cache[index(row + 1, column)].0, NextPixelLocation::Middle);

            if column != 0 {
                let left_disruption = cache[index(row + 1, column - 1)].0;

                if left_disruption < choice.0 {
                    choice = (left_disruption, NextPixelLocation::Left);
                }
            }

            if column != columns - 1 {
                let right_disruption = cache[index(row + 1, column + 1)].0;

                if right_disruption < choice.0 {
                    choice = (right_disruption, NextPixelLocation::Right);
                }
            }

            choice.0 += disruptions[index(row, column)];

            cache[index(row, column)] = choice;
        }
    }

    // Build result.

    let start = cache[..columns]
        .iter()
        .enumerate()
        .min_by(|(_, lhs), (_, rhs)| lhs.0.partial_cmp(&rhs.0).unwrap())
        .map(|(column, _)| (0, column));

    iter::successors(start, |&(row, column)| match cache[index(row, column)].1 {
        NextPixelLocation::None => None,
        NextPixelLocation::Left => Some((row + 1, column - 1)),
        NextPixelLocation::Middle => Some((row + 1, column)),
        NextPixelLocation::Right => Some((row + 1, column + 1)),
    })
    .map(|(_, column)| column)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::seam_carving;

    #[test]
    fn test_seam_carving() {
        let disruptions = [
            1.0, 1.0, 0.0, 1.0, 1.0, //
            1.0, 1.0, 0.0, 1.0, 1.0, //
            1.0, 0.0, 1.0, 1.0, 1.0, //
            1.0, 1.0, 0.0, 1.0, 1.0, //
        ];

        assert_eq!(*seam_carving(&disruptions, 5), [2, 2, 1, 2]);
    }
}
