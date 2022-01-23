use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter;

pub struct Edge<T, S> {
    pub observation: S,
    pub probability: f64,
    pub next: T,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    NoSuchPath,
}

#[allow(single_use_lifetimes)]
#[derive(PartialEq, Eq, Hash)]
struct CacheKey<'a, T> {
    node: &'a T,
    length: usize,
}

struct CacheValue<'a, T> {
    next: Option<&'a T>,
    probability: f64,
}

impl<T> Clone for CacheValue<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for CacheValue<'_, T> {}

fn get_most_probable_path_helper<'a, T: Eq + Hash, S: Eq, H: BuildHasher>(
    graph: &'a HashMap<T, Vec<Edge<T, S>>, H>,
    node: &'a T,
    observations: &[S],
    cache: &mut HashMap<CacheKey<'a, T>, Option<CacheValue<'a, T>>>,
) -> Option<CacheValue<'a, T>> {
    let key = CacheKey {
        node,
        length: observations.len(),
    };

    if let Some(&result) = cache.get(&key) {
        result
    } else {
        let choice = if let Some((first_observation, rest_observations)) = observations.split_first() {
            graph.get(node).and_then(|edges| {
                edges
                    .iter()
                    .filter_map(|edge| {
                        if edge.observation == *first_observation {
                            get_most_probable_path_helper(graph, &edge.next, rest_observations, cache).map(
                                |cache_value| CacheValue {
                                    next: Some(&edge.next),
                                    probability: cache_value.probability * edge.probability,
                                },
                            )
                        } else {
                            None
                        }
                    })
                    .max_by(|lhs, rhs| lhs.probability.partial_cmp(&rhs.probability).unwrap())
            })
        } else {
            Some(CacheValue {
                next: None,
                probability: 1.0,
            })
        };

        cache.insert(key, choice);

        choice
    }
}

/// # Errors
///
/// Will return `Error::NoSuchPath` if the path is not found.

pub fn get_most_probable_path<T: Eq + Hash + Clone, S: Eq, H: BuildHasher + Default>(
    graph: &HashMap<T, Vec<Edge<T, S>>, H>,
    start: &T,
    observations: &[S],
) -> Result<(Box<[T]>, f64), Error> {
    let mut cache = HashMap::new();

    get_most_probable_path_helper(graph, start, observations, &mut cache)
        .map(|cache_value| {
            let mut path = vec![start.clone()];

            path.extend(
                iter::successors(
                    cache_value.next.map(|next| CacheKey {
                        node: next,
                        length: observations.len() - 1,
                    }),
                    |cache_key| {
                        cache[cache_key].as_ref().unwrap().next.map(|next| CacheKey {
                            node: next,
                            length: cache_key.length - 1,
                        })
                    },
                )
                .map(|cache_key| cache_key.node.clone()),
            );

            (path.into(), cache_value.probability)
        })
        .ok_or(Error::NoSuchPath)
}

#[cfg(test)]
mod tests {
    use super::{get_most_probable_path, Edge, Error};
    use std::collections::HashMap;

    // https://en.wikipedia.org/wiki/Viterbi_algorithm.

    #[derive(PartialEq, Eq)]
    enum Observation {
        None,
        Normal,
        Cold,
        Dizzy,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    enum State {
        Start,
        Healthy,
        Fever,
    }

    fn get_graph() -> HashMap<State, Vec<Edge<State, Observation>>> {
        let mut graph = HashMap::new();

        graph.insert(
            State::Start,
            vec![
                Edge {
                    observation: Observation::None,
                    probability: 0.6,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::None,
                    probability: 0.4,
                    next: State::Fever,
                },
            ],
        );

        graph.insert(
            State::Healthy,
            vec![
                Edge {
                    observation: Observation::Normal,
                    probability: 0.4,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Cold,
                    probability: 0.25,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Dizzy,
                    probability: 0.05,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Normal,
                    probability: 0.1,
                    next: State::Fever,
                },
                Edge {
                    observation: Observation::Cold,
                    probability: 0.15,
                    next: State::Fever,
                },
                Edge {
                    observation: Observation::Dizzy,
                    probability: 0.05,
                    next: State::Fever,
                },
            ],
        );

        graph.insert(
            State::Fever,
            vec![
                Edge {
                    observation: Observation::Normal,
                    probability: 0.05,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Cold,
                    probability: 0.15,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Dizzy,
                    probability: 0.2,
                    next: State::Healthy,
                },
                Edge {
                    observation: Observation::Normal,
                    probability: 0.05,
                    next: State::Fever,
                },
                Edge {
                    observation: Observation::Cold,
                    probability: 0.15,
                    next: State::Fever,
                },
                Edge {
                    observation: Observation::Dizzy,
                    probability: 0.4,
                    next: State::Fever,
                },
            ],
        );

        graph
    }

    #[test]
    fn test_get_most_probable_path_ok() {
        let graph = get_graph();

        let (actual_states, actual_probability) = get_most_probable_path(
            &graph,
            &State::Start,
            &[
                Observation::None,
                Observation::Normal,
                Observation::Cold,
                Observation::Dizzy,
            ],
        )
        .unwrap();

        let expected_result = [State::Start, State::Healthy, State::Healthy, State::Fever, State::Fever];

        assert_eq!(*actual_states, expected_result);
        approx::assert_ulps_eq!(actual_probability, 0.0144);
    }

    #[test]
    fn test_get_most_probable_path_err() {
        let graph = get_graph();
        let result = get_most_probable_path(&graph, &State::Start, &[Observation::None, Observation::None]);

        assert_eq!(result, Err(Error::NoSuchPath));
    }
}
