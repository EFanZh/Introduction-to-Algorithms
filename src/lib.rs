#![warn(
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    // missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    // clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::panic_in_result_fn,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    // clippy::wildcard_dependencies
)]
#![allow(clippy::missing_panics_doc, clippy::non_ascii_literal)]

pub mod chapter_10_elementary_data_structures;
pub mod chapter_11_hash_tables;
pub mod chapter_12_binary_search_trees;
pub mod chapter_13_red_black_trees;
pub mod chapter_15_dynamic_programming;
pub mod chapter_16_greedy_algorithms;
pub mod chapter_17_amortized_analysis;
pub mod chapter_18_basic_operations_on_b_trees;
pub mod chapter_21_data_structures_for_disjoint_sets;
pub mod chapter_22_elementary_graph_algorithms;
pub mod chapter_2_getting_started;
pub mod chapter_4_divide_and_conquer;
pub mod chapter_5_probabilistic_analysis_and_randomized_algorithms;
pub mod chapter_6_heapsort;
pub mod chapter_7_quicksort;
pub mod chapter_8_sorting_in_linear_time;
pub mod chapter_9_medians_and_order_statistics;
pub mod chapter_c_counting_and_probability;

mod utilities;

#[cfg(test)]
mod test_utilities;
