use super::super::super::utilities::KeyValuePair;
use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{iter, mem, vec};

struct SkipSingle<I: Iterator> {
    iter: iter::Enumerate<I>,
    skip_minus_one: usize,
    skipped: Option<I::Item>,
}

impl<I: Iterator> SkipSingle<I> {
    fn new(mut iter: I, skip: usize) -> Self {
        if skip == 0 {
            let skipped = iter.next();

            Self {
                iter: iter.enumerate(),
                skip_minus_one: usize::MAX,
                skipped,
            }
        } else {
            Self {
                iter: iter.enumerate(),
                skip_minus_one: skip - 1,
                skipped: None,
            }
        }
    }
}

impl<I: Iterator> Iterator for SkipSingle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, item)| {
            if i == self.skip_minus_one {
                self.skipped = self.iter.next().map(|(_, next_item)| next_item);
            }

            item
        })
    }
}

pub struct DynamicArray<T> {
    data: Box<[Box<[T]>]>,
    len: usize,
    insertion_queue: BinaryHeap<KeyValuePair<Reverse<T>, vec::IntoIter<T>>>,
}

impl<T: Ord> Default for DynamicArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> DynamicArray<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Box::new([]),
            len: 0,
            insertion_queue: BinaryHeap::new(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn pop_insertion_queue(queue: &mut BinaryHeap<KeyValuePair<Reverse<T>, vec::IntoIter<T>>>) -> Option<T> {
        queue.pop().map(|mut item| {
            if let Some(next) = item.value.next() {
                queue.push(KeyValuePair::new(Reverse(next), item.value));
            }

            item.key.0
        })
    }

    fn merge_insertion_queue(
        queue: &mut BinaryHeap<KeyValuePair<Reverse<T>, vec::IntoIter<T>>>,
        size_hint: usize,
    ) -> Box<[T]> {
        let mut result = Vec::with_capacity(size_hint);

        result.push(Self::pop_insertion_queue(queue).unwrap());

        while let Some(item) = Self::pop_insertion_queue(queue) {
            result.push(item);
        }

        result.into()
    }

    pub fn insert(&mut self, value: T) {
        let q = &mut self.insertion_queue;

        q.push(KeyValuePair::new(Reverse(value), Vec::<T>::new().into_iter()));

        for (i, bucket) in self.data.iter_mut().enumerate() {
            let mut iter = mem::take(bucket).into_vec().into_iter();

            if let Some(key) = iter.next() {
                q.push(KeyValuePair::new(Reverse(key), iter));
            } else {
                *bucket = Self::merge_insertion_queue(q, 1 << i);

                self.len += 1;

                return;
            }
        }

        self.data = iter::repeat_with(Box::default).take(self.data.len() + 1).collect();

        let new_len = self.len + 1;

        *self.data.last_mut().unwrap() = Self::merge_insertion_queue(q, new_len);

        self.len = new_len;
    }

    pub fn search<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&T>
    where
        T: Borrow<Q>,
    {
        self.data.iter().find_map(|bucket| {
            bucket
                .binary_search_by_key(&key, T::borrow)
                .ok()
                .map(|index| &bucket[index])
        })
    }

    pub fn delete<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<T>
    where
        T: Borrow<Q>,
    {
        self.data
            .iter()
            .position(|bucket| !bucket.is_empty())
            .and_then(|pivot_index| {
                let (head, body) = self.data.split_at_mut(pivot_index);
                let (pivot, tail) = body.split_first_mut().unwrap();
                let mut pivot = mem::take(pivot).into_vec();

                if let Ok(element_index) = pivot.binary_search_by_key(&key, T::borrow) {
                    let mut pivot_iter = SkipSingle::new(pivot.into_iter(), element_index);

                    for (i, slot) in head.iter_mut().enumerate() {
                        *slot = pivot_iter.by_ref().take(1 << i).collect();
                    }

                    pivot_iter.skipped
                } else {
                    tail.iter_mut()
                        .find_map(|bucket| bucket.binary_search_by_key(&key, T::borrow).ok().map(|i| (bucket, i)))
                        .map(|(bucket, i)| {
                            let removed = mem::replace(&mut bucket[i], pivot.pop().unwrap());

                            bucket.sort_unstable();

                            let mut pivot_iter = pivot.into_iter();

                            for (i, slot) in head.iter_mut().enumerate() {
                                *slot = pivot_iter.by_ref().take(1 << i).collect();
                            }

                            removed
                        })
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{DynamicArray, SkipSingle};

    #[test]
    fn test_skip_single() {
        #[allow(trivial_casts)]
        let test_cases = [
            ((&[] as &[_], 0), (&[] as &[_], None)),
            ((&[], 2), (&[], None)),
            ((&[3, 2, 5], 0), (&[2, 5], Some(3))),
            ((&[3, 2, 5], 1), (&[3, 5], Some(2))),
            ((&[3, 2, 5], 2), (&[3, 2], Some(5))),
            ((&[3, 2, 5], 3), (&[3, 2, 5], None)),
            ((&[3, 2, 5], 4), (&[3, 2, 5], None)),
        ];

        for ((data, skip), (expected_result, expected_skipped)) in test_cases {
            let mut iter = SkipSingle::new(data.iter().copied(), skip);

            assert_eq!(iter.by_ref().collect::<Box<_>>().as_ref(), expected_result);
            assert_eq!(iter.skipped, expected_skipped);
        }
    }

    #[test]
    fn test_dynamic_array_insert() {
        #[allow(trivial_casts)]
        let test_cases = [
            (&[] as &[_], &[] as &[&[_]]),
            (&[3], &[&[3]]),
            (&[3, 4], &[&[], &[3, 4]]),
            (&[4, 3], &[&[], &[3, 4]]),
            (&[2, 3, 5], &[&[5], &[2, 3]]),
            (&[2, 5, 3], &[&[3], &[2, 5]]),
            (&[3, 2, 5], &[&[5], &[2, 3]]),
            (&[3, 5, 2], &[&[2], &[3, 5]]),
            (&[5, 2, 3], &[&[3], &[2, 5]]),
            (&[5, 3, 2], &[&[2], &[3, 5]]),
        ];

        for (nums, expected) in test_cases {
            let mut dynamic_array = DynamicArray::new();

            for num in nums {
                dynamic_array.insert(*num);
            }

            assert_eq!(
                *dynamic_array.data.iter().map(AsRef::as_ref).collect::<Box<_>>(),
                *expected
            );
        }
    }

    #[test]
    fn test_dynamic_array_search() {
        #[allow(trivial_casts)]
        let test_cases = [
            ((&[] as &[_], 2), None),
            ((&[3], 2), None),
            ((&[3], 3), Some(3)),
            ((&[3, 4], 2), None),
            ((&[3, 4], 3), Some(3)),
            ((&[3, 4], 4), Some(4)),
            ((&[4, 3], 2), None),
            ((&[4, 3], 3), Some(3)),
            ((&[4, 3], 4), Some(4)),
            ((&[2, 3, 5], 1), None),
            ((&[2, 3, 5], 2), Some(2)),
            ((&[2, 3, 5], 3), Some(3)),
            ((&[2, 3, 5], 4), None),
            ((&[2, 3, 5], 5), Some(5)),
            ((&[2, 3, 5], 6), None),
            ((&[2, 5, 3], 1), None),
            ((&[2, 5, 3], 2), Some(2)),
            ((&[2, 5, 3], 3), Some(3)),
            ((&[2, 5, 3], 4), None),
            ((&[2, 5, 3], 5), Some(5)),
            ((&[2, 5, 3], 6), None),
            ((&[3, 2, 5], 1), None),
            ((&[3, 2, 5], 2), Some(2)),
            ((&[3, 2, 5], 3), Some(3)),
            ((&[3, 2, 5], 4), None),
            ((&[3, 2, 5], 5), Some(5)),
            ((&[3, 2, 5], 6), None),
            ((&[3, 5, 2], 1), None),
            ((&[3, 5, 2], 2), Some(2)),
            ((&[3, 5, 2], 3), Some(3)),
            ((&[3, 5, 2], 4), None),
            ((&[3, 5, 2], 5), Some(5)),
            ((&[3, 5, 2], 6), None),
            ((&[5, 2, 3], 1), None),
            ((&[5, 2, 3], 2), Some(2)),
            ((&[5, 2, 3], 3), Some(3)),
            ((&[5, 2, 3], 4), None),
            ((&[5, 2, 3], 5), Some(5)),
            ((&[5, 2, 3], 6), None),
            ((&[5, 3, 2], 1), None),
            ((&[5, 3, 2], 2), Some(2)),
            ((&[5, 3, 2], 3), Some(3)),
            ((&[5, 3, 2], 4), None),
            ((&[5, 3, 2], 5), Some(5)),
            ((&[5, 3, 2], 6), None),
        ];

        for ((nums, target), expected) in test_cases {
            let mut dynamic_array = DynamicArray::new();

            for num in nums {
                dynamic_array.insert(*num);
            }

            assert_eq!(dynamic_array.search(&target).copied(), expected);
        }
    }

    #[test]
    fn test_dynamic_array_delete() {
        #[allow(trivial_casts)]
        let test_cases = [
            ((&[] as &[_], 2), None),
            ((&[3], 2), None),
            ((&[3], 3), Some(3)),
            ((&[3, 4], 2), None),
            ((&[3, 4], 3), Some(3)),
            ((&[3, 4], 4), Some(4)),
            ((&[4, 3], 2), None),
            ((&[4, 3], 3), Some(3)),
            ((&[4, 3], 4), Some(4)),
            ((&[2, 3, 5], 1), None),
            ((&[2, 3, 5], 2), Some(2)),
            ((&[2, 3, 5], 3), Some(3)),
            ((&[2, 3, 5], 4), None),
            ((&[2, 3, 5], 5), Some(5)),
            ((&[2, 3, 5], 6), None),
            ((&[2, 5, 3], 1), None),
            ((&[2, 5, 3], 2), Some(2)),
            ((&[2, 5, 3], 3), Some(3)),
            ((&[2, 5, 3], 4), None),
            ((&[2, 5, 3], 5), Some(5)),
            ((&[2, 5, 3], 6), None),
            ((&[3, 2, 5], 1), None),
            ((&[3, 2, 5], 2), Some(2)),
            ((&[3, 2, 5], 3), Some(3)),
            ((&[3, 2, 5], 4), None),
            ((&[3, 2, 5], 5), Some(5)),
            ((&[3, 2, 5], 6), None),
            ((&[3, 5, 2], 1), None),
            ((&[3, 5, 2], 2), Some(2)),
            ((&[3, 5, 2], 3), Some(3)),
            ((&[3, 5, 2], 4), None),
            ((&[3, 5, 2], 5), Some(5)),
            ((&[3, 5, 2], 6), None),
            ((&[5, 2, 3], 1), None),
            ((&[5, 2, 3], 2), Some(2)),
            ((&[5, 2, 3], 3), Some(3)),
            ((&[5, 2, 3], 4), None),
            ((&[5, 2, 3], 5), Some(5)),
            ((&[5, 2, 3], 6), None),
            ((&[5, 3, 2], 1), None),
            ((&[5, 3, 2], 2), Some(2)),
            ((&[5, 3, 2], 3), Some(3)),
            ((&[5, 3, 2], 4), None),
            ((&[5, 3, 2], 5), Some(5)),
            ((&[5, 3, 2], 6), None),
        ];

        for ((nums, target), expected) in test_cases {
            let mut dynamic_array = DynamicArray::new();

            for num in nums {
                dynamic_array.insert(*num);
            }

            assert_eq!(dynamic_array.delete(&target), expected);
        }
    }
}
