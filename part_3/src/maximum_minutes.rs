use super::*;

static DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
  pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut fire_time = vec![vec![-1; n]; m];
    let mut fire_visited = vec![vec![false; n]; m];
    let mut fire_bfs = vec![];
    for (i, row) in grid.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        if cell == 1 {
          fire_bfs.push((i as i32, j as i32));
          fire_time[i][j] = 0;
          fire_visited[i][j] = true;
        }
      }
    }
    let mut round = 1;
    while !fire_bfs.is_empty() {
      let count = fire_bfs.len();
      for i in 0..count {
        let pos = fire_bfs[i];
        for &dir in &DIRS {
          let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
          if next_pos.0 >= 0 && next_pos.1 >= 0 && next_pos.0 < m as i32 && next_pos.1 < n as i32 {
            let r = next_pos.0 as usize;
            let c = next_pos.1 as usize;
            if !fire_visited[r][c] && grid[r][c] != 2 {
              fire_visited[r][c] = true;
              fire_time[r][c] = round;
              fire_bfs.push(next_pos);
            }
          }
        }
      }
      round += 1;
      fire_bfs.drain(0..count);
    }
    let mut l = 0;
    let mut bfs = vec![];
    let mut r = if fire_time[0][0] == -1 {
      1e9 as i32
    } else {
      fire_time[0][0] - 1
    };
    let mut ret = -1;
    'a: while l <= r {
      let mid = (l + r) >> 1;
      round = mid + 1;
      let mut visited = vec![vec![false; n]; m];
      bfs.push((0, 0));
      while !bfs.is_empty() {
        let count = bfs.len();
        for i in 0..count {
          let pos = bfs[i];
          for &dir in &DIRS {
            let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if next_pos.0 >= 0 && next_pos.1 >= 0 && next_pos.0 < m as i32 && next_pos.1 < n as i32
            {
              let r = next_pos.0 as usize;
              let c = next_pos.1 as usize;
              if r + 1 == m && c + 1 == n && (fire_time[r][c] == -1 || fire_time[r][c] >= round) {
                if fire_time[r][c] == -1 || fire_time[r][c] >= round {
                  if ret == -1 || ret < mid {
                    ret = mid;
                    l = mid + 1;
                    bfs.clear();
                    continue 'a;
                  }
                }
              } else if !visited[r][c]
                && grid[r][c] != 2
                && (fire_time[r][c] == -1 || fire_time[r][c] > round)
              {
                visited[r][c] = true;
                bfs.push((r as i32, c as i32));
              }
            }
          }
        }
        bfs.drain(0..count);
        round += 1;
      }
      r = mid - 1;
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
  fn test_maximum_minutes_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![
          [0, 2, 0, 0, 0, 0, 0],
          [0, 0, 0, 2, 2, 1, 0],
          [0, 2, 0, 0, 1, 2, 0],
          [0, 0, 2, 2, 2, 0, 2],
          [0, 0, 0, 0, 0, 0, 0]
        ],
        ret: 3,
      },
      Suite {
        grid: t2_i![[0, 0, 0, 0], [0, 1, 2, 0], [0, 2, 0, 0]],
        ret: -1,
      },
      Suite {
        grid: t2_i![[0, 0, 0], [2, 2, 0], [1, 2, 0]],
        ret: 1000000000,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_minutes(s.grid));
    }
  }
}
