use super::*;

impl Solution {
  pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let placeholder = 10000;
    let n = matrix[0].len();
    let mut dp = vec![vec![placeholder; n]; 2];
    dp[0].copy_from_slice(&matrix[0]);
    for i in 0..matrix.len() - 1 {
      let index = i & 1;
      let next_index = 1 - index;
      for j in 0..n {
        if j > 0 {
          dp[next_index][j - 1] = dp[next_index][j - 1].min(dp[index][j] + matrix[i + 1][j - 1]);
        }
        dp[next_index][j] = dp[next_index][j].min(dp[index][j] + matrix[i + 1][j]);
        if j + 1 < n {
          dp[next_index][j + 1] = dp[next_index][j + 1].min(dp[index][j] + matrix[i + 1][j + 1]);
        }
      }
      dp[index].fill(placeholder);
    }
    *dp[(matrix.len() - 1) & 1].iter().min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_min_falling_path_sum_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[2, 1, 3], [6, 5, 4], [7, 8, 9]],
        ret: 13,
      },
      Suite {
        matrix: t2_i![[-19, 57], [-40, -5]],
        ret: -59,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_falling_path_sum(s.matrix));
    }
  }
}
