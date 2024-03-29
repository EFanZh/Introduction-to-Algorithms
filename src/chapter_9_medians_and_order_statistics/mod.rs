pub mod problems;
pub mod section_9_1_minimum_and_maximum;
pub mod section_9_2_selection_in_expected_linear_time;
pub mod section_9_3_selection_in_worst_case_linear_time;

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    pub fn run_all_select_test_cases<F: FnMut(&mut [i32], usize, usize, usize) -> &mut i32>(mut f: F) {
        let mut buffer = Vec::new();

        test_utilities::loop_on_all_unordered_sequences(|sequence, sorted_sequence| {
            for (i, expected_value) in sorted_sequence.iter().enumerate() {
                test_utilities::assign_vec(&mut buffer, sequence);

                assert_eq!(f(&mut buffer, 0, sequence.len(), i), expected_value);
            }
        });
    }
}
