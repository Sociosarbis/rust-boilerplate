use super::*;

impl Solution {
  pub fn get_money_amount(n: i32) -> i32 {
    let mut dp = vec![vec![0;n as usize];n as usize];
    let max = ((n + 1) * n) >> 1;
    for i in 1..n as usize {
      for j in (0..i).rev() {
        let mut min = max;
        for k in ((j + i) >> 1)..i + 1 {
          let left = if k > j { dp[j][k - 1] } else { 0 };
          let right = if k < i { dp[k + 1][i] } else { 0 };
          let temp = k as i32 + 1 + if left > right { left } else { right };
          if temp < min {
            min = temp;
          }
        }
        dp[j][i] = min;
      }
    }
    dp[0][n as usize - 1]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_get_money_amount_simple() {
    let suites = vec![
      Suite {
        n: 10,
        ret: 16
      },
      Suite {
        n: 1,
        ret: 0
      },
      Suite {
        n: 2,
        ret: 1
      },
      Suite {
        n: 6,
        ret: 8
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::get_money_amount(s.n));
    }
  }
}