use super::*;

impl Solution {
  pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let rows: Vec<bool> = grid
      .iter()
      .map(|r| {
        let mut ret = 0;
        for &cell in r {
          if cell == 1 {
            ret += 1;
            if ret > 1 {
              return true;
            }
          }
        }
        false
      })
      .collect();
    let cols: Vec<bool> = (0..grid[0].len())
      .map(|j| {
        let mut ret = 0;
        for row in &grid {
          if row[j] == 1 {
            ret += 1;
            if ret > 1 {
              return true;
            }
          }
        }
        false
      })
      .collect();
    grid.iter().enumerate().fold(0, |acc, (i, row)| {
      acc
        + row.iter().enumerate().fold(0, |acc, (j, &cell)| {
          acc
            + if cell == 1 && (rows[i] || cols[j]) {
              1
            } else {
              0
            }
        })
    })
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
  fn test_count_servers_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1, 0], [0, 1]],
        ret: 0,
      },
      Suite {
        grid: t2_i![[1, 0], [1, 1]],
        ret: 3,
      },
      Suite {
        grid: t2_i![[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]],
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_servers(s.grid));
    }
  }
}
