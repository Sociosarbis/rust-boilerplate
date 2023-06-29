use super::*;

impl Solution {
  pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![vec![0; colsum.len()]; 2];
    let empty = vec![];
    for (i, col) in colsum.into_iter().enumerate() {
      match col {
        2 => {
          if upper > 0 {
            upper -= 1;
          } else {
            return empty;
          }
          if lower > 0 {
            lower -= 1;
          } else {
            return empty;
          }
          ret[0][i] = 1;
          ret[1][i] = 1;
        }
        1 => {
          if upper >= lower {
            if upper > 0 {
              upper -= 1;
              ret[0][i] = 1;
            } else {
              return empty;
            }
          } else {
            if lower > 0 {
              lower -= 1;
              ret[1][i] = 1;
            } else {
              return empty;
            }
          }
        }
        _ => {}
      }
    }
    if lower == 0 && upper == 0 {
      ret
    } else {
      empty
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    upper: i32,
    lower: i32,
    colsum: Vec<i32>,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_reconstruct_matrix_simple() {
    let suites = vec![
      Suite {
        upper: 2,
        lower: 1,
        colsum: vec![1, 1, 1],
        ret: t2_i![[1, 1, 0], [0, 0, 1]],
      },
      Suite {
        upper: 2,
        lower: 3,
        colsum: vec![2, 2, 1, 1],
        ret: vec![],
      },
      Suite {
        upper: 5,
        lower: 5,
        colsum: vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1],
        ret: t2_i![
          [1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
          [1, 0, 1, 0, 1, 0, 0, 1, 0, 1]
        ],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::reconstruct_matrix(s.upper, s.lower, s.colsum)
      );
    }
  }
}
