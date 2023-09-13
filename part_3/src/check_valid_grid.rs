use super::*;

static DIRS: [(i32, i32); 8] = [
  (-1, -2),
  (-2, -1),
  (-2, 1),
  (-1, 2),
  (1, -2),
  (2, -1),
  (2, 1),
  (1, 2),
];

impl Solution {
  pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
      return false;
    }
    let mut steps: Vec<(i32, i32)> = vec![(0, 0); grid.len() * grid[0].len()];
    for (i, rows) in grid.iter().enumerate() {
      for (j, &order) in rows.iter().enumerate() {
        let index = order as usize;
        steps[index].0 = i as i32;
        steps[index].1 = j as i32;
      }
    }
    let mut pos = (0, 0);
    'a: for (row, col) in steps.into_iter().skip(1) {
      for &(dx, dy) in &DIRS {
        if pos.0 + dx == row && pos.1 + dy == col {
          pos = (row, col);
          continue 'a;
        }
      }
      return false;
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: bool,
  }

  #[test]
  fn test_check_valid_grid_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![
          [0, 11, 16, 5, 20],
          [17, 4, 19, 10, 15],
          [12, 1, 8, 21, 6],
          [3, 18, 23, 14, 9],
          [24, 13, 2, 7, 22]
        ],
        ret: true,
      },
      Suite {
        grid: t2_i![[0, 3, 6], [5, 8, 1], [2, 7, 4]],
        ret: false,
      },
      Suite {
        grid: t2_i![
          [24, 11, 22, 17, 4],
          [21, 16, 5, 12, 9],
          [6, 23, 10, 3, 18],
          [15, 20, 1, 8, 13],
          [0, 7, 14, 19, 2]
        ],
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_valid_grid(s.grid));
    }
  }
}
