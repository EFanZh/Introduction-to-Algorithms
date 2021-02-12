pub fn change_coins(coins: &[usize], n: usize) -> Box<[usize]> {
    let columns = n + 1;
    let mut cache = vec![None; columns * (coins.len() + 1)];

    cache[0] = Some((0, 0));

    let mut cache_iter = cache.chunks_exact_mut(columns);
    let mut previous_cache_line = cache_iter.next().unwrap();

    for (cache_line, coin) in cache_iter.zip(coins) {
        for i in 0..=n {
            cache_line[i] = match (
                previous_cache_line[i].map(|(count, _)| (count, i)),
                i.checked_sub(*coin)
                    .and_then(|j| cache_line[j].map(|(count, _)| (count + 1, j))),
            ) {
                (None, None) => None,
                (None, r @ Some(_)) | (r @ Some(_), None) => r,
                (Some((count_1, remain_1)), Some((count_2, remain_2))) => Some(if count_1 <= count_2 {
                    (count_1, remain_1)
                } else {
                    (count_2, remain_2)
                }),
            }
        }

        previous_cache_line = cache_line;
    }

    // Build result.

    let mut result = vec![0; coins.len()];
    let mut money = n;

    for (cache_line, target) in cache.chunks_exact(columns).skip(1).zip(result.iter_mut()).rev() {
        loop {
            let next = cache_line[money].unwrap().1;

            if next == money {
                break;
            }

            *target += 1;

            money = next;
        }
    }

    result.into()
}

#[cfg(test)]
mod tests {
    use super::change_coins;

    #[test]
    fn test_change_coins() {
        let test_cases = [
            ((&[] as &[_], 0), &[] as &[_]),
            ((&[25, 10, 5, 1], 0), &[0, 0, 0, 0]),
            ((&[25, 10, 5, 1], 1), &[0, 0, 0, 1]),
            ((&[25, 10, 5, 1], 2), &[0, 0, 0, 2]),
            ((&[25, 10, 5, 1], 5), &[0, 0, 1, 0]),
            ((&[25, 10, 5, 1], 6), &[0, 0, 1, 1]),
            ((&[25, 10, 5, 1], 7), &[0, 0, 1, 2]),
            ((&[25, 10, 5, 1], 10), &[0, 1, 0, 0]),
            ((&[25, 10, 5, 1], 11), &[0, 1, 0, 1]),
            ((&[25, 10, 5, 1], 12), &[0, 1, 0, 2]),
            ((&[25, 10, 5, 1], 15), &[0, 1, 1, 0]),
            ((&[25, 10, 5, 1], 16), &[0, 1, 1, 1]),
            ((&[25, 10, 5, 1], 17), &[0, 1, 1, 2]),
            ((&[25, 10, 5, 1], 25), &[1, 0, 0, 0]),
            ((&[25, 10, 5, 1], 26), &[1, 0, 0, 1]),
            ((&[25, 10, 5, 1], 27), &[1, 0, 0, 2]),
            ((&[25, 10, 5, 1], 30), &[1, 0, 1, 0]),
            ((&[25, 10, 5, 1], 31), &[1, 0, 1, 1]),
            ((&[25, 10, 5, 1], 32), &[1, 0, 1, 2]),
            ((&[25, 10, 5, 1], 35), &[1, 1, 0, 0]),
            ((&[25, 10, 5, 1], 36), &[1, 1, 0, 1]),
            ((&[25, 10, 5, 1], 37), &[1, 1, 0, 2]),
            ((&[25, 10, 5, 1], 40), &[1, 1, 1, 0]),
            ((&[25, 10, 5, 1], 41), &[1, 1, 1, 1]),
            ((&[25, 10, 5, 1], 42), &[1, 1, 1, 2]),
            ((&[1, 3, 4], 6), &[0, 2, 0]),
        ];

        for ((coins, n), expected) in test_cases.iter().copied() {
            assert_eq!(*change_coins(coins, n), *expected);
        }
    }
}
