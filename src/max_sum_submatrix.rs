use super::*;

impl Solution {
  pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp = vec![vec![vec![vec![0;matrix[0].len()];matrix.len()];matrix[0].len()];matrix.len()];
    let mut ret = k + 1;
    for i in 0..matrix.len() {
      for j in 0..matrix[0].len() {
        for m in i..matrix.len() {
          for n in j..matrix[0].len() {
            dp[i][j][m][n] = matrix[m][n];
            if m > i {
              if n > j {
                dp[i][j][m][n] += dp[i][j][m - 1][n] + dp[i][j][m][n - 1] - dp[i][j][m - 1][n - 1];
              } else {
                dp[i][j][m][n] += dp[i][j][m - 1][n];
              }
            } else if n > j {
              dp[i][j][m][n] += dp[i][j][m][n - 1];
            }
            if dp[i][j][m][n] <= k && (dp[i][j][m][n] > ret || ret > k) {
              ret = dp[i][j][m][n];
              if ret == k {
                break;
              }
            }
          }
        }
      }
    }
    ret
  }

  pub fn max_sum_submatrix_best(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut ret = k + 1;
    for i in 0..matrix.len() {
      let mut sum = vec![0;matrix[0].len()];
      for j in i..matrix.len() {
        for k in 0..matrix[0].len() {
          sum[k] += matrix[j][k];
        }

        let mut row_sum = vec![];
        let mut s = 0;
        for &num  in &sum {
          s += num;
          // 二分查找row_sum 中 符合大于等于 s - k的值，这样 s - row_sum[index]必定是小于等于k的最大值
          let mut index = Solution::binary_search(&row_sum, s - k, true) as usize;
          if index < row_sum.len() {
            if s - row_sum[index] > ret || ret > k {
              ret = s - row_sum[index];
              if ret == k {
                return ret;
              }
            }
          }
          if s <= k && (s  > ret || ret > k) {
            ret = s;
            if ret == k {
              return ret;
            }
          }
          index = Solution::binary_search(&row_sum, s, true) as usize;
          if index < row_sum.len() {
            row_sum.insert(index, s);
          } else {
            row_sum.push(s);
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
    matrix: Vec<Vec<i32>>,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_max_num_submatrix_simple() {
    let suites = vec![
      Suite {
        matrix: Solution::t2_i(vec![&[1,0,1],&[0,-2,3]]),
        k: 2,
        ret: 2,
      },
      Suite {
        matrix: Solution::t2_i(vec![&[2,2,-1]]),
        k: 3,
        ret: 3
      },
      Suite {
        matrix: Solution::t2_i(vec![&[5,-4,-3,4],&[-3,-4,4,5],&[5,1,5,-4]]),
        k: 3,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(Solution::max_sum_submatrix_best(s.matrix, s.k), s.ret);
    }
  }
}