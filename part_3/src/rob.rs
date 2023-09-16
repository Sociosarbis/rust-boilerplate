use super::*;

impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![[0; 2]; n];
    dp[0][1] = nums[0];
    for (i, num) in nums.into_iter().enumerate().skip(1) {
      dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
      dp[i][1] = dp[i - 1][0] + num;
    }
    dp[n - 1][0].max(dp[n - 1][1])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_rob_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 2, 3, 1],
        ret: 4,
      },
      Suite {
        nums: vec![2, 7, 9, 3, 1],
        ret: 12,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::rob(s.nums));
    }
  }
}
