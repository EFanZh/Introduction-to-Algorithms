fn build_result(cache: &[(usize, Option<usize>)], break_points: &[usize], i: usize, j: usize, result: &mut Vec<usize>) {
    let columns = break_points.len() + 1;

    if let Some(split) = cache[columns * i + j].1 {
        result.push(break_points[split]);

        build_result(cache, break_points, i, split, result);
        build_result(cache, break_points, split + 1, j, result);
    }
}

pub fn break_string(length: usize, break_points: &[usize]) -> (usize, Box<[usize]>) {
    let n = break_points.len();
    let columns = n + 1;
    let mut cache = vec![(0, None); columns * columns];

    for num_break_points in 1..=n {
        for i in 0..=n - num_break_points {
            let j = i + num_break_points;

            let mut choice = (i..i + num_break_points)
                .map(|k| (cache[columns * i + k].0 + cache[columns * (k + 1) + j].0, Some(k)))
                .min_by_key(|(cost, _)| *cost)
                .unwrap();

            let string_start = break_points.get(i.wrapping_sub(1)).copied().unwrap_or(0);
            let string_end = break_points.get(j).copied().unwrap_or(length);

            choice.0 += string_end - string_start;

            cache[columns * i + j] = choice;
        }
    }

    let cost = cache[n].0;
    let mut sequence = Vec::with_capacity(n);

    build_result(&cache, break_points, 0, n, &mut sequence);

    (cost, sequence.into())
}

#[cfg(test)]
mod tests {
    use super::break_string;

    #[test]
    fn test_break_string() {
        let (cost, break_sequence) = break_string(20, &[2, 8, 10]);

        assert_eq!(cost, 38);
        assert_eq!(*break_sequence, [10, 2, 8]);
    }
}
