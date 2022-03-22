#[allow(dead_code)]
struct NumMatrix {
  sum_left: Vec<Vec<i32>>,
  sum_right: Vec<Vec<i32>>
}

impl NumMatrix {
    #[allow(dead_code)]
    fn new(matrix: Vec<Vec<i32>>) -> Self {
      if matrix.is_empty() || matrix[0].is_empty() {
        return NumMatrix {
          sum_left: vec![],
          sum_right: vec![],
        }
      }
      let mut sum_left = vec![vec![0;matrix[0].len()];matrix.len()];
      let mut sum_right = vec![vec![0;matrix[0].len()];matrix.len()];
      for i in 0..matrix.len() {
        sum_left[i][0] = matrix[i][0];
        for j in 1..matrix[i].len() {
          sum_left[i][j] += sum_left[i][j - 1] + matrix[i][j];
        }
        sum_right[i][matrix[i].len() - 1] = matrix[i][matrix[i].len() - 1];
        for j in (0..matrix[i].len() - 1).rev() {
          sum_right[i][j] += sum_right[i][j + 1] + matrix[i][j];
        }
      }

      NumMatrix {
        sum_left: sum_left,
        sum_right: sum_right
      }
    }
    #[allow(dead_code)]
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
      if self.sum_left.is_empty() {
        return 0;
      }
      let mut ret = 0;
      for i in row1 as usize..row2 as usize + 1 {
        ret += self.sum_left[i][self.sum_left[i].len() - 1] -
        (if col1 > 0 { self.sum_left[i][col1 as usize - 1] } else { 0 })
        - (if col2 < self.sum_right[i].len() as i32 - 1 { self.sum_right[i][col2 as usize + 1] } else { 0 })
      }
      return ret;
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    regions: Vec<Vec<i32>>,
    rets: Vec<i32>
  }

  #[test]
  fn sum_region_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![
          [3, 0, 1, 4, 2],
          [5, 6, 3, 2, 1],
          [1, 2, 0, 1, 5],
          [4, 1, 0, 1, 7],
          [1, 0, 3, 0, 5]
        ],
        regions: t2_i![
          [2, 1, 4, 3],
          [1, 1, 2, 2],
          [1, 2, 2, 4]
        ],
        rets: vec![8,11,12]
      },
      Suite {
        matrix: t2_i![
          [-4, -5]
        ],
        regions: t2_i![
          [0,0,0,0],
          [0,0,0,1],
          [0,1,0,1]
        ],
        rets: vec![-4,-9,-5]
      }
    ];

    for s in suites {
      let num_matrix = NumMatrix::new(s.matrix);
      for i in 0..s.regions.len() {
        assert_eq!(num_matrix.sum_region(s.regions[i][0], s.regions[i][1], s.regions[i][2], s.regions[i][3]), s.rets[i]);
      }
    }
  }
}