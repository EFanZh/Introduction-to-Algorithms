use std::iter;

pub struct BaseballPlayer {
    cost: usize, // Unit: million dollars.
    vorp: u64,
}

pub fn sign_baseball_players(
    players: &[BaseballPlayer],
    players_per_position: usize,
    budget: usize,
) -> (u64, Box<[Option<usize>]>) {
    const CURRENT_PLAYER: usize = usize::MAX;

    let num_positions = players.len() / players_per_position;
    let cache_columns = budget + 1;
    let mut cache = vec![(0, 0); cache_columns * num_positions];

    // Fill the last position cache.

    let last_position_players = &players[players_per_position * (num_positions - 1)..];

    for (b, cache_item) in cache[cache_columns * (num_positions - 1)..].iter_mut().enumerate() {
        *cache_item = last_position_players
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if p.cost <= b { Some((p.vorp, i)) } else { None })
            .max_by_key(|(v, _)| *v)
            .unwrap_or((0, CURRENT_PLAYER));
    }

    // Fill remaining positions cache.

    for (position, players) in players[..players_per_position * (num_positions - 1)]
        .chunks_exact(players_per_position)
        .enumerate()
        .rev()
    {
        let (position_cache, next_position_cache) = cache[cache_columns * position..].split_at_mut(cache_columns);

        for (b, cache_item) in position_cache
            .iter_mut()
            .enumerate()
            .skip(if position == 0 { budget } else { 0 })
        {
            *cache_item = players
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    b.checked_sub(p.cost)
                        .map(|remain_budget| (p.vorp + next_position_cache[remain_budget].0, i))
                })
                .chain(iter::once((next_position_cache[b].0, CURRENT_PLAYER)))
                .max_by_key(|(v, _)| *v)
                .unwrap();
        }
    }

    // Build result.

    (
        cache[budget].0,
        cache
            .chunks_exact(cache_columns)
            .zip(players.chunks_exact(players_per_position))
            .scan(budget, |b, (cache_row, player_row)| {
                let player_index = cache_row[*b].1;

                Some(if player_index == CURRENT_PLAYER {
                    None
                } else {
                    *b -= player_row[player_index].cost;

                    Some(player_index)
                })
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::{sign_baseball_players, BaseballPlayer};

    #[test]
    fn test_sign_baseball_players() {
        let players = [
            [(4, 20), (1, 40), (1, 30), (2, 10)],
            [(1, 30), (5, 30), (2, 40), (0, 00)],
            [(5, 10), (5, 10), (1, 30), (1, 40)],
            [(2, 00), (4, 40), (5, 00), (5, 10)],
            [(4, 20), (2, 20), (5, 30), (3, 30)],
            [(1, 00), (1, 50), (5, 50), (5, 00)],
            [(0, 10), (4, 40), (5, 20), (5, 20)],
            [(2, 10), (3, 10), (4, 30), (3, 00)],
            [(5, 00), (5, 40), (2, 30), (5, 40)],
        ];

        let players_per_position = 4;
        let budget = 15;

        let (total_vorp, selected_players) = sign_baseball_players(
            &*players
                .iter()
                .flat_map(|position_players| {
                    position_players
                        .iter()
                        .map(|&(cost, vorp)| BaseballPlayer { cost, vorp })
                })
                .collect::<Box<_>>(),
            players_per_position,
            budget,
        );

        assert_eq!(total_vorp, 280);

        let expected_selected_players = [
            Some(1),
            Some(2),
            Some(3),
            Some(1),
            None,
            Some(1),
            Some(1),
            None,
            Some(2),
        ];

        assert_eq!(*selected_players, expected_selected_players);
    }
}
