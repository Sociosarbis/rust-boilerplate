use super::*;

use std::mem;

impl Solution {
  pub fn tiling_rectangle(mut n: i32, mut m: i32) -> i32 {
    if n > m {
      mem::swap(&mut n, &mut m);
    }
    let mut dp = vec![vec![0; m as usize + 1]; n as usize + 1];
    Solution::tiling_rectangle_dfs(&mut dp, n as usize, m as usize)
  }

  fn tiling_rectangle_dfs(dp: &mut Vec<Vec<i32>>, mut n: usize, mut m: usize) -> i32 {
    if n > m {
      mem::swap(&mut n, &mut m);
    }
    if n == 0 || m == 0 {
      return 0;
    }
    if n == m {
      return 1;
    }
    if n == 1 {
      return m as i32;
    }
    if dp[n][m] != 0 {
      return dp[n][m];
    }
    let mut ret = (n * m) as i32;
    // 矩形拆分
    for i in 1..=n {
      ret = ret.min(
        1 + Solution::tiling_rectangle_dfs(dp, n, m - i)
          + Solution::tiling_rectangle_dfs(dp, n - i, i),
      );

      ret = ret.min(
        1 + Solution::tiling_rectangle_dfs(dp, i, m - i)
          + Solution::tiling_rectangle_dfs(dp, n - i, m),
      );
      for j in n - i + 1..=(m - i).min(n) {
        ret = ret.min(
          2 + Solution::tiling_rectangle_dfs(dp, n - i, m - j)
            + Solution::tiling_rectangle_dfs(dp, n - j, m - i)
            + Solution::tiling_rectangle_dfs(dp, i + j - n, m - i - j),
        );
      }
    }
    dp[n][m] = ret;
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    m: i32,
    ret: i32,
  }

  #[test]
  fn test_tiling_rectangle_simple() {
    let suites = vec![Suite { n: 2, m: 3, ret: 3 }, Suite { n: 5, m: 8, ret: 5 }];

    for s in suites {
      assert_eq!(s.ret, Solution::tiling_rectangle(s.n, s.m));
    }
  }
}
