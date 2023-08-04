use super::*;

impl Solution {
  pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let mut empty_cells = 0;
    let n = grid[0].len();
    let mut start = (0, 0);
    for (i, row) in grid.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        if cell == 1 {
          start.0 = i;
          start.1 = j;
        }
        if cell != -1 {
          empty_cells |= 1 << (i * n + j);
        }
      }
    }
    Solution::unique_paths_iii_dfs(
      &grid,
      start.0,
      start.1,
      1 << (start.0 * n + start.1),
      empty_cells,
    )
  }

  fn unique_paths_iii_dfs(
    grid: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    visited_cells: i32,
    empty_cells: i32,
  ) -> i32 {
    if grid[i][j] == 2 {
      if visited_cells == empty_cells {
        return 1;
      }
    }
    let mut ret = 0;
    let mut options = Vec::with_capacity(4);
    if i > 0 {
      options.push((i - 1, j));
    }
    if i + 1 < grid.len() {
      options.push((i + 1, j));
    }
    if j > 0 {
      options.push((i, j - 1));
    }
    if j + 1 < grid[0].len() {
      options.push((i, j + 1));
    }
    for (next_i, next_j) in options {
      let bits = 1 << (next_i * grid[0].len() + next_j);
      if bits & visited_cells == 0 {
        if grid[next_i][next_j] != -1 {
          ret +=
            Solution::unique_paths_iii_dfs(grid, next_i, next_j, visited_cells | bits, empty_cells);
        }
      }
    }
    ret
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
  fn test_unique_paths_iii_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]],
        ret: 2,
      },
      Suite {
        grid: t2_i![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]],
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::unique_paths_iii(s.grid));
    }
  }
}
