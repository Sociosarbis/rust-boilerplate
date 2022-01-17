use super::*;

impl Solution {
  pub fn count_vowel_permutation(n: i32) -> i32 {
    let mut dp = vec![vec![1;5], vec![0;5]];
    let modulo = 1000000007;
    for i in 1..n as usize {
      let index = i & 1;
      dp[index][0] = ((dp[1 - index][1] + dp[1 - index][4]) % modulo + dp[1 - index][2]) % modulo;
      dp[index][1] = (dp[1 - index][0] + dp[1 - index][2]) % modulo;
      dp[index][2] = (dp[1 - index][1] + dp[1 - index][3]) % modulo;
      dp[index][3] = dp[1 - index][2];
      dp[index][4] = (dp[1 - index][3] + dp[1 - index][2]) % modulo; 
    }
    dp[((n - 1) & 1) as usize].iter().fold(0, |acc, item| (acc + item) % modulo)
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
  fn test_count_vowel_permutation_simple() {
    let suites = vec![
      Suite {
        n: 1,
        ret: 5
      },
      Suite {
        n: 2,
        ret: 10
      },
      Suite {
        n: 5,
        ret: 68
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_vowel_permutation(s.n));
    }
  }
}