use super::*;

impl Solution {
  pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
      let mut l = 0;
      let mut r = matrix.len() - 1;
      while l <= r {
        let mid  = (l + r) >> 1;
        let start = matrix[mid][0];
        let end = *matrix[mid].last().unwrap();
        if start <= target && end >= target {
          l = mid;
          r = mid;
          while l > 0 && matrix[l-1][0] <= target && *matrix[l-1].last().unwrap() >= target {
            l -= 1;
          }
          while r < matrix.len() - 1 && matrix[r+1][0] <= target && *matrix[r+1].last().unwrap() >= target {
            r += 1;
          }
          for i in l..r+1 {
            if Solution::binary_search(&matrix[i], target, false) != -1 {
              return true;
            }
          }
          break;
        } else if target < start {
          if mid > 0 {
            r = mid - 1;
          } else {
            break;
          }
        } else {
          l = mid + 1;
        }
      }
      false
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
        matrix: t2_i![[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]],
        target: 5,
        ret: true
      },
      Suite {
        matrix: t2_i![[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]],
        target: 20,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::search_matrix(s.matrix, s.target));
    }
  }
}