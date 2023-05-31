use super::*;

impl Solution {
  pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    if arr.len() == 2 {
      return arr[0] * arr[1];
    }
    let n = arr.len();
    let mut dp = vec![vec![(0, 0); n]; n];
    for (i, &num) in arr.iter().enumerate() {
      dp[i][i].1 = num;
    }
    for i in 2..=n {
      for j in 0..=n - i {
        let mut best = (0, 0);
        for k in 1..i {
          let temp = (
            dp[j][j + k - 1].0
              + dp[j + k][j + i - 1].0
              + dp[j][j + k - 1].1 * dp[j + k][j + i - 1].1,
            dp[j][j + k - 1].1.max(dp[j + k][j + i - 1].1),
          );
          if best.0 == 0 || (temp.0 < best.0 || (temp.0 == best.0 && temp.1 < best.1)) {
            best = temp;
          }
        }
        dp[j][j + i - 1] = best;
      }
    }
    dp[0][n - 1].0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_mct_from_leaf_values_simple() {
    let suites = vec![
      Suite {
        arr: vec![6, 2, 4],
        ret: 32,
      },
      Suite {
        arr: vec![4, 11],
        ret: 44,
      },
      Suite {
        arr: vec![11, 12, 12],
        ret: 276,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::mct_from_leaf_values(s.arr));
    }
  }
}
