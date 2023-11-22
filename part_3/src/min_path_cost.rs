use super::*;

impl Solution {
  pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; 2];
    dp[0].copy_from_slice(&grid[0]);
    for (i, row) in grid.iter().enumerate().take(m - 1) {
      let index = i & 1;
      let next_index = 1 - index;
      dp[next_index].fill(0);
      for (j, &cell) in row.iter().enumerate() {
        for k in 0..n {
          let next_value = dp[index][j] + grid[i + 1][k] + move_cost[cell as usize][k];
          if dp[next_index][k] == 0 || dp[next_index][k] > next_value {
            dp[next_index][k] = next_value;
          }
        }
      }
    }
    *dp[(m - 1) & 1].iter().min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    move_cost: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_min_path_cost_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[5, 3], [4, 0], [2, 1]],
        move_cost: t2_i![[9, 8], [1, 5], [10, 12], [18, 6], [2, 4], [14, 3]],
        ret: 17,
      },
      Suite {
        grid: t2_i![[5, 1, 2], [4, 0, 3]],
        move_cost: t2_i![
          [12, 10, 15],
          [20, 23, 8],
          [21, 7, 1],
          [8, 1, 13],
          [9, 10, 25],
          [5, 3, 2]
        ],
        ret: 6,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_path_cost(s.grid, s.move_cost));
    }
  }
}
