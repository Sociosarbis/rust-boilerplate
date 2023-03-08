use super::*;

impl Solution {
  pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len();
    let mut dp = vec![vec![0;n];2];
    for (i, row) in grid.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        let top = if i > 0 { dp[(i - 1) & 1][j] } else { 0 };
        let left = if j > 0 { dp[i & 1][j - 1] } else { 0 };
        dp[i & 1][j] = cell + if top > left { top } else { left };
      }
    }
    dp[(grid.len() - 1) & 1][n - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_max_value_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![
          [1,3,1],
          [1,5,1],
          [4,2,1]
        ],
        ret: 12
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_value(s.grid));
    }
  }
}