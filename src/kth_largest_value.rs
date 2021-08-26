use super::*;

impl Solution {
  pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut arr = vec![];
    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..m {
      for j in 0..n {
        let mut num = matrix[i][j];
        if i != 0 {
          num ^= arr[(i - 1) * n + j];
        }
        if j != 0 {
          num ^= arr[i * n + j - 1];
        }
        if i != 0 && j != 0 {
          num ^= arr[(i - 1) * n + j - 1];
        }
        arr.push(num);
      }
    }
    arr.sort();
    arr[arr.len() - k as usize]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_kth_largest_value_simple() {
    let suites = vec![
      Suite {
        matrix: Solution::t2_i(vec![&[5,2],&[1,6]]),
        k: 1,
        ret: 7,
      },
      Suite {
        matrix: Solution::t2_i(vec![&[5,2],&[1,6]]),
        k: 2,
        ret: 5,
      },
      Suite {
        matrix: Solution::t2_i(vec![&[5,2],&[1,6]]),
        k: 3,
        ret: 4,
      },
      Suite {
        matrix: Solution::t2_i(vec![&[5,2],&[1,6]]),
        k: 4,
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(Solution::kth_largest_value(s.matrix, s.k), s.ret);
    }
  }
}