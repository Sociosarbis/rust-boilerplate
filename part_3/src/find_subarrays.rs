use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn find_subarrays(nums: Vec<i32>) -> bool {
    if nums.len() > 2 {
      let mut m = HashSet::new();
      for i in 1..nums.len() {
        let sum = nums[i] + nums[i - 1];
        if m.contains(&sum) {
          return true;
        }
        m.insert(sum);
      }
    }
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: bool
  }

  #[test]
  fn test_find_subarrays_simple() {
    let suites = vec![
      Suite {
        nums: vec![4,2,4],
        ret: true,
      },
      Suite {
        nums: vec![1,2,3,4,5],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_subarrays(s.nums));
    }
  }
}