use super::*;

impl Solution {
  pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut ret = vec![vec![0;n-2];n-2];
    for i in 0..n-2 {
      for j in 0..n-2 {
        for k in 0..3 {
          for l in 0..3 {
            if grid[i + k][j + l] > ret[i][j] {
              ret[i][j] = grid[i + k][j + l];
            }
          }
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
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn test_largest_local_simple() {
    let suites = vec![
      Suite {
        grid: t2_i![[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]],
        ret: t2_i![[2,2,2],[2,2,2],[2,2,2]],
      },
      Suite {
        grid: t2_i![[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]],
        ret: t2_i![[9,9],[8,6]],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::largest_local(s.grid));
    }
  }
}