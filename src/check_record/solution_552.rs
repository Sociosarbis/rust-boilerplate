use crate::Solution;

impl Solution {
  pub fn check_record_552(n: i32) -> i32 {
    let mut dp: [[i32;3];2] = [[0;3];2];
    dp[0][0] = 1;
    for _ in 1..n + 1 {
      let mut new_dp = [[0;3];2];
      for i in 0..2 {
        for j in 0..3 {
          if dp[i][j] != 0 {
            let count = dp[i][j];
            if i + 1 < 2 {
              new_dp[i + 1][0] = (new_dp[i + 1][0] + count) % 1000000007;
            }
            new_dp[i][0] = (new_dp[i][0] + count) % 1000000007;
            if j + 1 < 3 {
              new_dp[i][j + 1] = (new_dp[i][j + 1] + count) % 1000000007;
            }
          }
        }
      }
      dp = new_dp;
    }
    dp.iter().fold(0, |acc, item| { (acc + item.iter().fold(0, |acc, item| { (acc + item) % 1000000007 })) % 1000000007 })
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_check_records_552_simple() {
    let suites = vec![
      Suite {
        n: 2,
        ret: 8
      },
      Suite {
        n: 1,
        ret: 3
      },
      Suite {
        n: 10101,
        ret: 183236316
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_record_552(s.n));
    }
  }
}