use super::*;

static DIRS: [(i32, i32); 8] = [
  (0, -1),
  (1, -1),
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 1),
  (-1, 0),
  (-1, -1),
];

impl Solution {
  pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    if r == 1 && c == 1 && grid[0][0] == 0 {
      return 1;
    }
    if grid[0][0] == 0 && grid[r - 1][c - 1] == 0 {
      let mut bfs: Vec<(i32, i32)> = vec![(0, 0)];
      let mut visited = vec![vec![false; c]; r];
      let mut temp = 1;
      while !bfs.is_empty() {
        let n = bfs.len();
        temp += 1;
        for i in 0..n {
          for &dir in &DIRS {
            let (next_i, next_j) = (bfs[i].0 + dir.0, bfs[i].1 + dir.1);
            if next_i >= 0 && next_i < r as i32 && next_j >= 0 && next_j < c as i32 {
              let (n_i, n_j) = (next_i as usize, next_j as usize);
              if grid[n_i][n_j] == 0 {
                if n_i + 1 == r && n_j + 1 == c {
                  return temp;
                }
                if !visited[n_i][n_j] {
                  visited[n_i][n_j] = true;
                  bfs.push((next_i, next_j));
                }
              }
            }
          }
        }
        bfs.drain(0..n);
      }
    }
    -1
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
  fn test_shortest_path_binary_matrix_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[0, 1], [1, 0]],
        ret: 2,
      },
      Suite {
        grid: t2_i![[0, 0, 0], [1, 1, 0], [1, 1, 0]],
        ret: 4,
      },
      Suite {
        grid: t2_i![[1, 0, 0], [1, 1, 0], [1, 1, 0]],
        ret: -1,
      },
      Suite {
        grid: t2_i![[0]],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::shortest_path_binary_matrix(s.grid));
    }
  }
}
