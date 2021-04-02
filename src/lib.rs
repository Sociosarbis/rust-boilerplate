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

  pub fn binary_search(nums: &Vec<i32>, target: i32, is_insert: bool) -> i32 {
    if nums.is_empty() {
      return if is_insert { 0 } else { -1 };
    }
    let mut l = 0;
    let mut r = nums.len() - 1;
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
