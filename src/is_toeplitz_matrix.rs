use super::*;

use std::cmp::min;

impl Solution {
  pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
      let m = matrix.len();
      let n = matrix[0].len();

      for i in 0..n {
        let num = matrix[0][i];
        for j in 1..min(n - i, m) {
          if num != matrix[j][i + j] {
            return false;
          }
        }
      }

      for i in 1..m {
        let num = matrix[i][0];
        for j in 1..min(m - i, n) {
          if num != matrix[i + j][j] {
            return false;
          }
        }
      }
      return true;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    ret: bool
  }

  #[test]
  fn is_toeplitz_matrix_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[1,2,3,4],[5,1,2,3],[9,5,1,2]],
        ret: true
      },
      Suite {
        matrix: t2_i![[1,2],[2,2]],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(Solution::is_toeplitz_matrix(s.matrix), s.ret);
    }
  }
}