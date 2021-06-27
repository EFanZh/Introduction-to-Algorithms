#[derive(Clone, Copy)]
enum Choice {
    Empty,
    Single,
    Wrap,
    DelegateLeft,
    DelegateRight,
}

pub fn longest_palindrome_subsequence<T: Eq + Clone>(s: &[T]) -> Box<[T]> {
    let n = s.len();
    let mut cache = vec![(0, Choice::Empty); s.len() * (s.len() + 1)];

    for i in 0..s.len() {
        cache[n + i] = (1, Choice::Single);
    }

    for length in 2..=s.len() {
        for i in 0..=s.len() - length {
            cache[n * length + i] = if s[i] == s[i + (length - 1)] {
                (cache[n * (length - 2) + (i + 1)].0 + 2, Choice::Wrap)
            } else {
                let length_1 = cache[n * (length - 1) + i].0;
                let length_2 = cache[n * (length - 1) + (i + 1)].0;

                if length_2 > length_1 {
                    (length_2, Choice::DelegateRight)
                } else {
                    (length_1, Choice::DelegateLeft)
                }
            };
        }
    }

    let mut length = s.len();
    let mut i = 0;
    let mut result = Vec::new();
    let mut stack = Vec::new();

    loop {
        match cache[n * length + i].1 {
            Choice::Empty => break,
            Choice::Single => {
                result.push(s[i].clone());
                break;
            }
            Choice::Wrap => {
                result.push(s[i].clone());
                stack.push(i + (length - 1));
                length -= 2;
                i += 1;
            }
            Choice::DelegateLeft => {
                length -= 1;
            }
            Choice::DelegateRight => {
                length -= 1;
                i += 1;
            }
        }
    }

    result.extend(stack.into_iter().rev().map(|i| s[i].clone()));

    result.into()
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome_subsequence;

    #[test]
    fn test_longest_palindrome_subsequence() {
        #[allow(trivial_casts)]
        let test_cases = [
            (b"bbbab" as &[_], b"bbbb" as &[_]),
            (b"cbbd", b"bb"),
            (b"character", b"carac"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(*longest_palindrome_subsequence(s), *expected);
        }
    }
}
