use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut ret = 0;
    let i = nums.iter().enumerate().find(|(_, &num)| num == k).map(|(j, &_)| j).unwrap();
    let mut left = HashMap::new();
    let mut temp = 0;
    left.insert(0, 1);
    for j in (0..i).rev() {
      if nums[j] < k {
        temp -= 1;
      } else {
        temp += 1;
      }
      if let Some(c) = left.get_mut(&temp) {
        *c += 1;
      } else {
        left.insert(temp, 1);
      }
    }
    temp = 0;
    for j in i..nums.len() {
      if nums[j] > k {
        temp += 1;
      } else if nums[j] < k {
        temp -= 1;
      }
      if let Some(&c) = left.get(&-temp) {
        ret += c;
      }
      if let Some(&c) = left.get(&(-temp + 1)) {
        ret += c;
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
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_count_subarrays_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,2,1,4,5],
        k: 4,
        ret: 3
      },
      Suite {
        nums: vec![2,3,1],
        k: 3,
        ret: 1
      },
      Suite {
        nums: vec![5,19,11,15,13,16,4,6,2,7,10,8,18,20,1,3,17,9,12,14],
        k: 6,
        ret: 13
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_subarrays(s.nums, s.k)); 
    }
  }
}