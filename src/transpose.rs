use super::*;

impl Solution {
  pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![0;matrix.len()];matrix[0].len()];
    for i in 0..matrix.len() {
      let row = &matrix[i];
      for j in 0..matrix[0].len() {
        ret[j][i] = row[j];
      }
    }
    return ret;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn transpose_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[1,2,3],[4,5,6],[7,8,9]],
        ret: t2_i![[1,4,7],[2,5,8],[3,6,9]]
      },
      Suite {
        matrix: t2_i![[1,2,3],[4,5,6]],
        ret: t2_i![[1,4],[2,5],[3,6]]
      }
    ];
    for s in suites {
      assert_eq!(Solution::transpose(s.matrix), s.ret);
    }
  }
}