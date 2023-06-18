use super::*;

impl Solution {
  pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
      for j in 0..n {
        if grid[i][j] == 0 && Solution::closed_island_dfs(&mut grid, i, j) {
          ret += 1;
        }
      }
    }
    ret
  }

  fn closed_island_dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    if grid[i][j] == 0 {
      let m = grid.len();
      let n = grid[0].len();
      grid[i][j] = 2;
      let mut ret = !(i == 0 || i + 1 == m || j == 0 || j + 1 == n);
      if i > 0 {
        ret &= Solution::closed_island_dfs(grid, i - 1, j);
      };
      if i + 1 != m {
        ret &= Solution::closed_island_dfs(grid, i + 1, j);
      }
      if j > 0 {
        ret &= Solution::closed_island_dfs(grid, i, j - 1);
      }
      if j + 1 != n {
        ret &= Solution::closed_island_dfs(grid, i, j + 1);
      }
      ret
    } else {
      true
    }
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
  fn test_closed_island_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![
          [1, 1, 1, 1, 1, 1, 1, 0],
          [1, 0, 0, 0, 0, 1, 1, 0],
          [1, 0, 1, 0, 1, 1, 1, 0],
          [1, 0, 0, 0, 0, 1, 0, 1],
          [1, 1, 1, 1, 1, 1, 1, 0]
        ],
        ret: 2,
      },
      Suite {
        grid: t2_i![[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]],
        ret: 1,
      },
      Suite {
        grid: t2_i![
          [1, 1, 1, 1, 1, 1, 1],
          [1, 0, 0, 0, 0, 0, 1],
          [1, 0, 1, 1, 1, 0, 1],
          [1, 0, 1, 0, 1, 0, 1],
          [1, 0, 1, 1, 1, 0, 1],
          [1, 0, 0, 0, 0, 0, 1],
          [1, 1, 1, 1, 1, 1, 1]
        ],
        ret: 2,
      },
      Suite {
        grid: t2_i![
          [0, 0, 1, 1, 0, 1, 0, 0, 1, 0],
          [1, 1, 0, 1, 1, 0, 1, 1, 1, 0],
          [1, 0, 1, 1, 1, 0, 0, 1, 1, 0],
          [0, 1, 1, 0, 0, 0, 0, 1, 0, 1],
          [0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
          [0, 1, 0, 1, 0, 1, 0, 1, 1, 1],
          [1, 0, 1, 0, 1, 1, 0, 0, 0, 1],
          [1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
          [1, 1, 1, 0, 0, 1, 0, 1, 0, 1],
          [1, 1, 1, 0, 1, 1, 0, 1, 1, 0]
        ],
        ret: 5,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::closed_island(s.grid));
    }
  }
}
