use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix[0].len();
    let mut temp: Vec<u8> = vec![0;n];
    let mut counter = HashMap::new();
    for row in matrix {
      if row[0] == 1 {
        for (i, &value) in row.iter().enumerate() {
          temp[i] = value as u8;
        }
      } else {
        for (i, &value) in row.iter().enumerate() {
          temp[i] = (1 - value) as u8;
        }
      }
      if let Some(c) = counter.get_mut(&temp) {
        *c += 1;
      } else {
        counter.insert(temp.clone(), 1);
      }
    }
    *counter.values().max().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_max_equal_rows_after_flip_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[0,1],[1,1]],
        ret: 1
      },
      Suite {
        matrix: t2_i![[0,1],[1,0]],
        ret: 2
      },
      Suite {
        matrix: t2_i![[0,0,0],[0,0,1],[1,1,0]],
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_equal_rows_after_flips(s.matrix));
    }
  }
}