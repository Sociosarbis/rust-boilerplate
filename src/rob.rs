use super::Solution;

use std::cmp::max;

impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0;2];2];
    dp[0][0] = nums[0];
    let mut next = 1;
    for i in 1..nums.len() - 1 {
      dp[next][0] = nums[i] + dp[1 - next][1];
      dp[next][1] = max(dp[1 - next][0], dp[1 - next][1]);
      next = 1 - next;
    }
    let temp_max_1 = max(dp[1 - next][0], dp[1 - next][1]);
    if nums.len() > 1 {
      dp[0][0] = nums[1];
      dp[0][1] = 0;
      next = 1;
      for i in 2..nums.len() {
        dp[next][0] = nums[i] + dp[1 - next][1];
        dp[next][1] = max(dp[1 - next][0], dp[1 - next][1]);
        next = 1 - next;
      }
      let temp_max_2 = max(dp[1 - next][0], dp[1 - next][1]);
      return max(temp_max_1, temp_max_2);
    }
    temp_max_1
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
  fn test_rob_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,3,2],
        ret: 3
      },
      Suite {
        nums: vec![1,2,3,1],
        ret: 4
      },
      Suite {
        nums: vec![0],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(Solution::rob(s.nums), s.ret);
    }
  }
}