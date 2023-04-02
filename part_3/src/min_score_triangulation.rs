use super::*;

impl Solution {
  pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let n = values.len();
    let mut dp = vec![vec![0;n];n];
    for i in 0..n - 2 {
      dp[i][i + 2] = values[i] * values[i + 1] * values[i + 2];
    }
    for i in 3..n {
      for j in 0..n - i {
        for k in 1..i {
          let temp = values[j] * values[j + i] * values[j + k] + dp[j][j + k] + dp[j + k][j + i];
          if dp[j][j + i] == 0 || temp < dp[j][j + i] {
            dp[j][j + i] = temp;
          }
        }
      }
    }
    dp[0][n - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    values:  Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_min_score_triangulation_simple() {
    let suites = vec![
      Suite {
        values: vec![1,2,3],
        ret: 6
      },
      Suite {
        values: vec![3,7,4,5],
        ret: 144
      },
      Suite {
        values: vec![1,3,1,4,1,5],
        ret: 13
      },
      Suite {
        values: vec![2,1,4,4],
        ret: 24
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_score_triangulation(s.values));
    }
  }
}