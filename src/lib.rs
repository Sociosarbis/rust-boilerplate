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
}
