use super::*;


impl Solution {
  pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    if k == 0 {
      return 1;
    }
    let mut dp = vec![vec![0;k as usize + 1];2];
    dp[0][0] = 1;
    dp[1][0] = 1;
    for i in 1..n as usize {
      let idx = i as usize & 1;
      for j in 1..k as usize + 1 {
        let prev_l = if j - 1 < i { 0 } else { j - 1 - i };
        let cur_l = if j < i { 0 } else { j - i };
        dp[idx][j] = (dp[idx][j - 1] + dp[1 - idx][j]) % 1000000007;
        if cur_l != prev_l {
          dp[idx][j] += if dp[idx][j] < dp[1 - idx][prev_l] { 1000000007 } else { 0 } - dp[1 - idx][prev_l];
        }
      }
    }
    dp[(n - 1) as usize & 1][k as usize]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    k: i32,
    ret: i32
  }

  #[test]
  fn test_k_inverse_pairs_simple() {
    let suites = vec![
      Suite {
        n: 3,
        k: 0,
        ret: 1
      },
      Suite {
        n: 3,
        k: 1,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::k_inverse_pairs(s.n, s.k));
    }
  }
}