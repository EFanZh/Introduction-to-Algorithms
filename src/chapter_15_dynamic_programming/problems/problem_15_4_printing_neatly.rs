#[derive(Clone)]
struct Choice {
    cost: usize,
    next_split: usize,
}

fn build_initial_cache(word_lengths: &[usize], columns: usize) -> (Box<[Choice]>, usize) {
    let cache = vec![Choice { cost: 0, next_split: 0 }; word_lengths.len()];

    let remaining_words = word_lengths
        .split_last()
        .and_then(|(&last_word_length, word_lengths)| {
            let mut line_length = last_word_length;

            word_lengths
                .iter()
                .copied()
                .rposition(|length| {
                    line_length += length + 1;

                    line_length > columns
                })
                .map(|x| x + 1)
        })
        .unwrap_or(0);

    (cache.into(), remaining_words)
}

fn cube(x: usize) -> usize {
    x * x * x
}

fn build_cut_positions(cache: &[Choice]) -> Box<[usize]> {
    let mut result = Vec::new();
    let mut i = 0;

    while let Some(choice) = cache.get(i) {
        if choice.next_split == 0 {
            break;
        }

        result.push(choice.next_split);

        i = choice.next_split;
    }

    result.into()
}

pub fn printing_neatly(word_lengths: &[usize], columns: usize) -> Box<[usize]> {
    let (mut cache, remaining_words) = build_initial_cache(word_lengths, columns);

    for (i, length_i) in word_lengths.iter().copied().enumerate().take(remaining_words).rev() {
        let mut line_extra_spaces = columns.saturating_sub(length_i);

        let mut best_choice = Choice {
            cost: cube(line_extra_spaces) + cache[i + 1].cost,
            next_split: i + 1,
        };

        for (j, length_j) in word_lengths.iter().copied().enumerate().skip(i + 1) {
            if let Some(new_line_extra_spaces) = line_extra_spaces.checked_sub(length_j + 1) {
                let new_cost = cube(new_line_extra_spaces) + cache[j + 1].cost;

                // Use greedy strategy.

                if new_cost <= best_choice.cost {
                    best_choice.cost = new_cost;
                    best_choice.next_split = j + 1;
                }

                line_extra_spaces = new_line_extra_spaces;
            } else {
                break;
            }
        }

        cache[i] = best_choice;
    }

    build_cut_positions(&cache)
}

pub fn layout_paragraph_neatly(words: &[&str], columns: usize) -> Box<[String]> {
    let mut result = Vec::new();
    let word_lengths = words.iter().map(|s| s.chars().count()).collect::<Box<_>>();
    let cuts = printing_neatly(&word_lengths, columns);

    let mut previous_cut = 0;

    for cut in cuts.iter().copied() {
        result.push(words[previous_cut..cut].join(" "));

        previous_cut = cut;
    }

    result.push(words[previous_cut..].join(" "));

    result.into()
}

#[cfg(test)]
mod tests {
    use super::{layout_paragraph_neatly, printing_neatly};

    #[test]
    fn test_printing_neatly() {
        assert_eq!(*printing_neatly(&[], 10), []);
        assert_eq!(*printing_neatly(&[2], 10), []);
        assert_eq!(*printing_neatly(&[2, 3], 10), []);
        assert_eq!(*printing_neatly(&[2, 3, 5], 10), [2]);
        assert_eq!(*printing_neatly(&[2, 3, 5, 7], 10), [2, 3]);
        assert_eq!(*printing_neatly(&[2, 3, 5, 7], 7), [2, 3]);
    }

    const WORDS: &[&str] = &[
        "Consider",
        "the",
        "problem",
        "of",
        "neatly",
        "printing",
        "a",
        "paragraph",
        "with",
        "a",
        "monospaced",
        "font",
        "(all",
        "characters",
        "having",
        "the",
        "same",
        "width)",
        "on",
        "a",
        "printer.",
        "The",
        "input",
        "text",
        "is",
        "a",
        "sequence",
        "of",
        "*n*",
        "words",
        "of",
        "lengths",
        "$l_1$,",
        "$l_2$,",
        "…,",
        "$l_n$,",
        "measured",
        "in",
        "characters.",
        "We",
        "want",
        "to",
        "print",
        "this",
        "paragraph",
        "neatly",
        "on",
        "a",
        "number",
        "of",
        "lines",
        "that",
        "hold",
        "a",
        "maximum",
        "of",
        "*M*",
        "characters",
        "each.",
        "Our",
        "criterion",
        "of",
        "“neatness”",
        "is",
        "as",
        "follows.",
        "If",
        "a",
        "given",
        "line",
        "contains",
        "words",
        "*i*",
        "through",
        "*j*,",
        "where",
        "*i* ≤ *j*,",
        "and",
        "we",
        "leave",
        "exactly",
        "one",
        "space",
        "between",
        "words,",
        "the",
        "number",
        "of",
        "extra",
        "space",
        "characters",
        "at",
        "the",
        "end",
        "of",
        "the",
        "line",
        "is",
        "*M* - *j* + *i* - $∑_{k = i}^j l_k$,",
        "which",
        "must",
        "be",
        "nonnegative",
        "so",
        "that",
        "the",
        "words",
        "fit",
        "on",
        "the",
        "line.",
        "We",
        "wish",
        "to",
        "minimize",
        "the",
        "sum,",
        "over",
        "all",
        "lines",
        "except",
        "the",
        "last,",
        "of",
        "the",
        "cubes",
        "of",
        "the",
        "numbers",
        "of",
        "extra",
        "space",
        "characters",
        "at",
        "the",
        "ends",
        "of",
        "lines.",
        "Give",
        "a",
        "dynamic-programming",
        "algorithm",
        "to",
        "print",
        "a",
        "paragraph",
        "of",
        "*n*",
        "words",
        "neatly",
        "on",
        "a",
        "printer.",
        "Analyze",
        "the",
        "running",
        "time",
        "and",
        "space",
        "requirements",
        "of",
        "your",
        "algorithm.",
    ];

    #[test]
    fn test_layout_paragraph_neatly() {
        let result = layout_paragraph_neatly(WORDS, 60);

        let expected_result = [
            "Consider the problem of neatly printing a paragraph with",
            "a monospaced font (all characters having the same width)",
            "on a printer. The input text is a sequence of *n* words of",
            "lengths $l_1$, $l_2$, …, $l_n$, measured in characters. We",
            "want to print this paragraph neatly on a number of lines",
            "that hold a maximum of *M* characters each. Our criterion",
            "of “neatness” is as follows. If a given line contains words",
            "*i* through *j*, where *i* ≤ *j*, and we leave exactly one",
            "space between words, the number of extra space characters at",
            "the end of the line is *M* - *j* + *i* - $∑_{k = i}^j l_k$,",
            "which must be nonnegative so that the words fit on the line.",
            "We wish to minimize the sum, over all lines except the last,",
            "of the cubes of the numbers of extra space characters at the",
            "ends of lines. Give a dynamic-programming algorithm to print",
            "a paragraph of *n* words neatly on a printer. Analyze the",
            "running time and space requirements of your algorithm.",
        ];

        assert_eq!(*result, expected_result);
    }
}
