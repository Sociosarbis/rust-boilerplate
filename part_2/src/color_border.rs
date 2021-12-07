use super::*;

static DIRS: [(i32, i32);4] = [(-1,0),(1,0),(0,-1),(0,1)];
impl Solution {
  pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![vec![false;grid[0].len()];grid.len()];
    let mut ret = grid.clone();
    let mut bfs = vec![(row, col)];
    let orig_color = grid[row as usize][col as usize];
    while !bfs.is_empty() {
      if let Some((i, j)) = bfs.pop() {
        for dir in &DIRS {
          let next_i = i + dir.0;
          let next_j = j + dir.1;
          if next_i >= 0
            && next_i < grid.len() as i32
            && next_j >=0
            && next_j < grid[0].len() as i32
            && !visited[next_i as usize][next_j as usize]
            && grid[next_i as usize][next_j as usize] == orig_color {
              visited[next_i as usize][next_j as usize] = true;
              bfs.push((next_i, next_j));
          }
        }
        if i == 0
          || j == 0
          || i == grid.len() as i32 - 1
          || j == grid[0].len() as i32 - 1
          || DIRS.iter().find(|dir| grid[(i + dir.0) as usize][(j + dir.1) as usize] != orig_color).is_some() {
            ret[i as usize][j as usize] = color;
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
    row: i32,
    col: i32,
    color: i32,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn test_color_border_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1,1],[1,2]],
        row: 0,
        col: 0,
        color: 3,
        ret: t2_i![[3,3],[3,2]]
      },
      Suite {
        grid: t2_i![[1,2,2],[2,3,2]],
        row: 0,
        col: 1,
        color: 3,
        ret: t2_i![[1,3,3],[2,3,3]]
      },
      Suite {
        grid: t2_i![[1,1,1],[1,1,1],[1,1,1]],
        row: 1,
        col: 1,
        color: 2,
        ret: t2_i![[2,2,2],[2,1,2],[2,2,2]]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::color_border(s.grid, s.row, s.col, s.color));
    }
  }
}