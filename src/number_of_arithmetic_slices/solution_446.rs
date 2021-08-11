use crate::Solution;

use std::collections::HashMap;

impl Solution {
  pub fn number_of_arithmetic_slices_446(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
      return 0;
    }
    let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new();nums.len()];
    for i in (2..nums.len()).rev() {
      for j in 1..i {
        let delta = nums[i] as i64 - nums[j] as i64;
        if !dp[i].contains_key(&delta) {
          dp[i].insert(delta, 1);
        }
      }
      for j in i + 1..nums.len() {
        let delta = nums[j] as i64 - nums[i] as i64;
        if dp[i].contains_key(&delta) {
          *dp[i].get_mut(&delta).unwrap() += *dp[j].get(&delta).unwrap();
        }
      }
    }
    let mut ret = 0;
    for i in 1..nums.len() - 1 {
      let mut counter: HashMap<i64, i32> = HashMap::new(); 
      for j in 0..i {
        let delta = nums[i] as i64 - nums[j] as i64;
        if !counter.contains_key(&delta) {
          counter.insert(delta, 0);
        }
        *counter.get_mut(&delta).unwrap() += 1;
      }
      for j in i + 1..nums.len() {
        let delta = nums[j] as i64 - nums[i] as i64;
        if let Some(&count) = counter.get(&delta) {
          ret += *dp[j].get(&delta).unwrap() * count;
        }
      }
    }
    ret
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
  fn test_number_of_arithmetic_slices_446_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,4,6,8,10],
        ret: 7
      },
      Suite {
        nums: vec![7,7,7,7,7],
        ret: 16
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::number_of_arithmetic_slices_446(s.nums));
    }
  }
}