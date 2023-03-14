use super::*;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
      let m = row_sum.len();
      let n = col_sum.len();
      let mut ret = vec![vec![0;n];m];
      for i in 0..m {
        for j in 0..n {
          let v = if row_sum[i] <= col_sum[j] { row_sum[i] } else { col_sum[j] };
          row_sum[i] -= v;
          col_sum[j] -= v;
          ret[i][j] = v;
        }
      }
      ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        row_sum: Vec<i32>,
        col_sum: Vec<i32>,
        ret: Vec<Vec<i32>>,
    }

    #[test]
    fn test_restore_matrix_simple() {
        let suites = vec![
            Suite {
                row_sum: vec![3, 8],
                col_sum: vec![4, 7],
                ret: t2_i![[3, 0], [1, 7]],
            },
            Suite {
                row_sum: vec![5, 7, 10],
                col_sum: vec![8, 6, 8],
                ret: t2_i![[5, 0, 0], [3, 4, 0], [0, 2, 8]],
            },
        ];

        for s in suites {
          assert_eq!(s.ret, Solution::restore_matrix(s.row_sum, s.col_sum));
        }
    }
}
