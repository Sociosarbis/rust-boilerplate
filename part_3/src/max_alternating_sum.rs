use super::*;

impl Solution {
  pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut dp: [i64; 2] = [0;2];
    for num in nums {
      let next_dp = [dp[1] - num as i64, dp[0] + num as i64];
      dp[0] = dp[0].max(next_dp[0]);
      dp[1] = dp[1].max(next_dp[1]);
    }
    dp[0].max(dp[1])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i64,
  }

  #[test]
  fn test_max_alternating_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![4, 2, 5, 3],
        ret: 7,
      },
      Suite {
        nums: vec![5, 6, 7, 8],
        ret: 8,
      },
      Suite {
        nums: vec![6, 2, 1, 2, 4, 5],
        ret: 10,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_alternating_sum(s.nums));
    }
  }
}
