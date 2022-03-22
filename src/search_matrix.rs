use super::*;

impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut l1 = 0;
    let mut r1 = m - 1;
    while l1 < r1 {
      let mid = (l1 + r1) / 2;
      if target >= matrix[mid][0] && target <= matrix[mid][n - 1] {
        l1 = mid;
        r1 = mid;
      } else if target < matrix[mid][0] {
        if mid == 0 {
          return false;
        } else {
          r1 = mid - 1;
        }
      } else {
        if mid == m - 1 {
          return false;
        } else {
          l1 = mid + 1;
        }
      }
    }
    if target >= matrix[l1][0] && target <= matrix[l1][n - 1] {
      let i = Solution::binary_search(&matrix[l1], target, false);
      if i == -1 {
        return false;
      } else {
        return true;
      };
    } else {
      return false;
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    matrix: Vec<Vec<i32>>,
    target: i32,
    ret: bool
  }

  #[test]
  fn test_search_matrix_simple() {
    let suites = vec![
      Suite {
        matrix: t2_i![[1,3,5,7],[10,11,16,20],[23,30,34,60]],
        target: 3,
        ret: true,
      },
      Suite {
        matrix: t2_i![[1,3,5,7],[10,11,16,20],[23,30,34,60]],
        target: 13,
        ret: false
      }
    ];
    for s in suites {
      assert_eq!(Solution::search_matrix(s.matrix, s.target), s.ret);
    }
  }
}