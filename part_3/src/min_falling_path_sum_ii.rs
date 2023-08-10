use super::*;

impl Solution {
  pub fn min_falling_path_sum_ii(grid: Vec<Vec<i32>>) -> i32 {
    let placeholder = 20000;
    let n = grid[0].len();
    let mut dp = vec![vec![placeholder; n]; 2];
    dp[0].copy_from_slice(&grid[0]);
    for i in 0..grid.len() - 1 {
      let index = i & 1;
      let next_index = 1 - index;
      for j in 0..n {
        for k in 0..n {
          if j != k {
            dp[next_index][k] = dp[next_index][k].min(dp[index][j] + grid[i + 1][k]);
          }
        }
      }
      dp[index].fill(placeholder);
    }
    *dp[(grid.len() - 1) & 1].iter().min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_min_falling_path_sum_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        ret: 13,
      },
      Suite {
        grid: t2_i![[7]],
        ret: 7,
      },
      Suite {
        grid: t2_i![
          [-73, 61, 43, -48, -36],
          [3, 30, 27, 57, 10],
          [96, -76, 84, 59, -15],
          [5, -49, 76, 31, -7],
          [97, 91, 61, -46, 67]
        ],
        ret: -192,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_falling_path_sum_ii(s.grid));
    }
  }
}
