use super::*;

impl Solution {
  pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let size = if m > n { n } else { m };
    let mut prefix_row = vec![vec![0;n + 1];m];
    let mut prefix_col = vec![vec![0;m + 1];n];
    for (i, row) in grid.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        prefix_row[i][j + 1] = cell + prefix_row[i][j];
      }
    }
    for i in 0..n {
      for j in 0..m {
        prefix_col[i][j + 1] = prefix_col[i][j] + grid[j][i];
      }
    }
    for i in (1..=size).rev() {
      for j in 0..=m-i {
        for k in 0..=n-i {
            let r1 = (prefix_row[j][k + i] - prefix_row[j][k]) as usize;
            if r1 == i {
              let r2 = (prefix_row[j + i - 1][k + i] - prefix_row[j + i - 1][k]) as usize;
              if r2 == i {
                let c1 = (prefix_col[k][j + i] - prefix_col[k][j]) as usize;
                if c1 == i {
                  let c2 = (prefix_col[k + i - 1][j + i] - prefix_col[k + i - 1][j]) as usize;
                  if c2 == i {
                    return (i * i) as i32;
                  }
                }
              }
            }
        }
      }
    }
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    grid: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_largest1_bordered_square_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1,1,1],[1,0,1],[1,1,1]],
        ret: 9
      },
      Suite {
        grid: t2_i![[1,1,0,0]],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::largest1_bordered_square(s.grid));
    }
  }
}