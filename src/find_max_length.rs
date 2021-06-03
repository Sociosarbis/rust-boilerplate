use super::Solution;

use std::collections::HashMap;

impl Solution {
  pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut diff_to_index: HashMap<i32, usize> = HashMap::new();
    let mut diff = 0;
    let mut ret = 0;
    diff_to_index.insert(diff, 0);
    for i in 0..nums.len() {
      if nums[i] == 0 {
        diff -= 1;
      } else {
        diff += 1;
      }
      if let Some(index) = diff_to_index.get(&diff) {
        let next_ret = i + 1 - index;
        if next_ret > ret {
          ret = next_ret;
        }
      } else {
        diff_to_index.insert(diff, i + 1);
      }
    }
    ret as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_find_max_length_simple() {
    let suites = vec![
      Suite {
        nums: vec![0, 1],
        ret: 2,
      },
      Suite {
        nums: vec![0, 1, 0],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_max_length(s.nums));
    }
  }
}