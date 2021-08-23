use super::*;

impl Solution {
  pub fn get_maximum_generated(n: i32) -> i32 {
    if n < 2 {
      return n;
    }
    let mut dp = vec![0;n as usize + 1];
    dp[1] = 1;
    let mut ret = 1;
    for i in 2..n as usize + 1 {
      dp[i] = if i & 1 == 1 { dp[i >> 1] + dp[(i >> 1) + 1] } else { dp[i >> 1] };
      if dp[i] > ret {
        ret = dp[i];
      }
    }
    ret
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
  fn test_get_maximum_generated_simple() {
    let suites = vec![
      Suite {
        n: 7,
        ret: 3
      },
      Suite {
        n: 2,
        ret: 1
      },
      Suite {
        n: 3,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::get_maximum_generated(s.n));
    }
  }
}