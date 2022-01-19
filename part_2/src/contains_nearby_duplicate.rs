use super::*;

use std::collections::HashSet;


impl Solution {
  pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    let n = if nums.len() < k as usize + 1 { nums.len() } else { k as usize + 1 };
    for i in 0..n {
      if !set.insert(nums[i]) {
        return true
      }
    }
    for i in n..nums.len() {
      set.remove(&nums[i - n]);
      if !set.insert(nums[i]) {
        return true
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
    k: i32,
    ret: bool
  }

  #[test]
  fn test_contains_nearby_duplicate_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,1],
        k: 3,
        ret: true
      },
      Suite {
        nums: vec![1,0,1,1],
        k: 1,
        ret: true
      },
      Suite {
        nums: vec![1,2,3,1,2,3],
        k: 2,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::contains_nearby_duplicate(s.nums, s.k));
    }
  }
}