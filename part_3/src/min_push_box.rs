use super::*;
use std::collections::HashMap;

static DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
  pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut start_box = 0;
    let mut start_player = 0;
    let mut target = 0;
    for (i, row) in grid.iter().enumerate() {
      for (j, cell) in row.iter().enumerate() {
        match cell {
          'B' => {
            start_box = i * n + j;
          }
          'S' => {
            start_player = i * n + j;
          }
          'T' => {
            target = i * n + j;
          }
          _ => {}
        }
      }
    }
    visited.insert((start_box as i32, start_player as i32), 0);
    let mut bfs = vec![(start_box as i32, start_player as i32)];
    let is_valid = |pos: (i32, i32)| {
      pos.0 >= 0
        && pos.0 < m as i32
        && pos.1 >= 0
        && pos.1 < n as i32
        && grid[pos.0 as usize][pos.1 as usize] != '#'
    };
    let mut ret = -1;
    while !bfs.is_empty() {
      let len = bfs.len();
      for i in 0..len {
        let box_pos = (bfs[i].0 / n as i32, bfs[i].0 % n as i32);
        let player_pos = (bfs[i].1 / n as i32, bfs[i].1 % n as i32);
        let value = *visited.get(&bfs[i]).unwrap();
        for dir in &DIRS {
          let next_player_pos = (player_pos.0 + dir.0, player_pos.1 + dir.1);
          if is_valid(next_player_pos) {
            if box_pos == next_player_pos {
              let next_box_pos = (box_pos.0 + dir.0, box_pos.1 + dir.1);
              if is_valid(next_box_pos) {
                let state = (
                  next_box_pos.0 * n as i32 + next_box_pos.1,
                  next_player_pos.0 * n as i32 + next_player_pos.1,
                );
                if state.0 == target as i32 {
                  if ret == -1 || ret > value + 1 {
                    ret = value + 1;
                  }
                } else if let Some(c) = visited.get_mut(&state) {
                  if *c > value + 1 {
                    *c = value + 1;
                    bfs.push(state);
                  }
                } else {
                  visited.insert(state, value + 1);
                  bfs.push(state);
                }
              }
            } else {
              let state = (
                box_pos.0 * n as i32 + box_pos.1,
                next_player_pos.0 * n as i32 + next_player_pos.1,
              );
              if let Some(c) = visited.get_mut(&state) {
                if *c > value {
                  *c = value;
                  bfs.push(state);
                }
              } else {
                visited.insert(state, value);
                bfs.push(state);
              }
            }
          }
        }
      }
      bfs.drain(0..len);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<char>>,
    ret: i32,
  }

  #[test]
  fn test_min_push_box_simple() {
    let suites = vec![
     /*  Suite {
        grid: t2_c![
          ["#", "#", "#", "#", "#", "#"],
          ["#", "T", "#", "#", "#", "#"],
          ["#", ".", ".", "B", ".", "#"],
          ["#", ".", "#", "#", ".", "#"],
          ["#", ".", ".", ".", "S", "#"],
          ["#", "#", "#", "#", "#", "#"]
        ],
        ret: 3,
      },
      Suite {
        grid: t2_c![
          ["#", "#", "#", "#", "#", "#"],
          ["#", "T", "#", "#", "#", "#"],
          ["#", ".", ".", "B", ".", "#"],
          ["#", "#", "#", "#", ".", "#"],
          ["#", ".", ".", ".", "S", "#"],
          ["#", "#", "#", "#", "#", "#"]
        ],
        ret: -1,
      },
      Suite {
        grid: t2_c![
          ["#", "#", "#", "#", "#", "#"],
          ["#", "T", ".", ".", "#", "#"],
          ["#", ".", "#", "B", ".", "#"],
          ["#", ".", ".", ".", ".", "#"],
          ["#", ".", ".", ".", "S", "#"],
          ["#", "#", "#", "#", "#", "#"]
        ],
        ret: 5,
      },
      Suite {
        grid: t2_c![
          ["#", ".", ".", "#", "#", "#", "#", "#"],
          ["#", ".", ".", "T", "#", ".", ".", "#"],
          ["#", ".", ".", ".", "#", "B", ".", "#"],
          ["#", ".", ".", ".", ".", ".", ".", "#"],
          ["#", ".", ".", ".", "#", ".", "S", "#"],
          ["#", ".", ".", "#", "#", "#", "#", "#"]
        ],
        ret: 7,
      }, */
      Suite {
        grid: t2_c![
          [".", ".", "#", ".", ".", ".", ".", "#"],
          [".", "B", ".", ".", ".", ".", ".", "#"],
          [".", ".", "S", ".", ".", ".", ".", "."],
          [".", "#", ".", ".", ".", ".", ".", "."],
          [".", ".", ".", ".", ".", ".", ".", "."],
          [".", ".", ".", "T", ".", ".", ".", "."],
          [".", ".", ".", ".", ".", ".", ".", "#"],
          [".", "#", ".", ".", ".", ".", ".", "."]
        ],
        ret: 6,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_push_box(s.grid));
    }
  }
}
