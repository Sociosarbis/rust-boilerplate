mod monotone_increasing_digits;
mod unique_paths;
mod word_pattern;
mod max_profit;
mod find_the_difference;
mod min_cost_climbing_stairs;
mod zigzag_level_order;
mod candy;
mod max_profit_4;
mod min_patches;
mod last_stone_weight;
mod erase_overlap_intervals;
mod fib;
mod large_group_positions;
mod calc_equation;
mod find_circle_num;
mod rotate;
mod smallest_string_with_swaps;
mod sort_items;
mod find_redundant_connection;
mod prefixes_div_by5;
mod remove_stones;
mod accounts_merge;
mod min_cost_connect_points;
mod maximum_product;
mod find_critical_and_pseudo_critical_edges;
mod add_to_array_form;
mod regions_by_slashes;
mod num_equiv_domino_pairs;
mod max_num_edges_to_remove;
mod pivot_index;
mod minimum_effort_path;
mod fair_candy_swap;
mod median_sliding_window;
mod find_max_average;
mod equal_substring;
mod longest_ones;
mod find_shortest_sub_array;
mod longest_subarray;
mod is_toeplitz_matrix;
mod max_satisfied;
mod flip_and_invert_image;
mod transpose;
mod sum_range;
mod sum_region;
mod count_bits;
mod max_envelopes;
mod min_cut;
mod remove_duplicates;
mod calculate;
mod calculate2;
mod is_valid_serialization;
mod spiral_order;
mod generate_matrix;
mod num_distinct;
mod parking_system;
mod reverse_between;
mod hamming_weight;
mod nested_iterator;
mod find132pattern;
mod delete_duplicates;
mod delete_duplicates_2;
mod reverse_bits;
mod search_matrix;
mod clumsy;
mod trap;
mod search;
mod find_min;
mod largest_number;
mod min_diff_in_bst;
mod trie;
mod rob;
mod is_scramble;
mod remove_element;
mod str_str;
mod num_decodings;
mod max_sum_submatrix;
mod largest_divisible_subset;
mod increasing_bst;
mod ship_within_days;
mod range_sum_bst;
mod judge_square_sum;
mod can_cross;
mod single_number;
mod decode;
mod xor_operation;
mod minimum_time_required;
mod leaf_similar;
mod xor_queries;
mod num_ways;
mod int_to_roman;
mod is_cousins;
mod count_triplets;
mod kth_largest_value;
mod top_k_frequent;
mod max_uncrossed_lines;
mod strange_printer;
mod min_changes;
mod reverse_parentheses;
mod hamming_distance;
mod total_hamming_distance;
mod is_power_of_four;
mod can_eat;
mod check_subarray_sum;
mod find_max_length;
mod max_points;
mod open_lock;
mod num_buses_to_destination;
mod convert_to_title;
mod max_ice_cream;
mod count_of_atoms;
mod display_table;
mod count_pairs;
mod num_subarrays_with_sum;
mod majority_element;
mod time_map;
mod h_index;
mod get_skyline;
mod min_absolute_sum_diff;
mod maximum_element_after_decrementing_and_rearranging;
mod search_lcof;
mod max_frequency;
mod min_pair_nums;
mod is_covered;
mod min_operations;

pub struct Solution {}

impl Solution {
  pub fn t1(source: Vec<&str>) -> Vec<String> {
    source.into_iter().map(|s| { s.to_owned() }).collect()
  }

  pub fn t2(source: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    source.into_iter().map(|a| {
      Solution::t1(a)
    }).collect()
  }

  pub fn t1_i(source: &[i32]) -> Vec<i32> {
    source.to_owned()
  }

  pub fn t2_i(source: Vec<&[i32]>) -> Vec<Vec<i32>> {
    source.into_iter().map(|s| { Solution::t1_i(s)}).collect()
  }

  pub fn binary_search<T: Ord + Eq>(nums: &Vec<T>, target: T, is_insert: bool) -> i32 {
    if nums.is_empty() {
      return if is_insert { 0 } else { -1 };
    }
    return Solution::binary_search_general(nums, target, 0, nums.len() - 1, is_insert)
  }

  pub fn binary_search_general<T: Ord + Eq>(nums: &Vec<T>, target: T, i: usize, j: usize, is_insert: bool) -> i32 {
    let mut l = i;
    let mut r = j;
    while l <= r {
      let mid = (l + r) / 2;
      if nums[mid] < target {
        l = mid + 1;
        if l > r {
          return if is_insert { l as i32 } else { -1 }
        }
      } else if nums[mid] > target {
        if mid == 0 || mid - 1 < l {
          return if is_insert { mid as i32 } else { -1 };
        }
        r = mid - 1;
      } else {
        return mid as i32;
      }
    }
    0
  }

}
