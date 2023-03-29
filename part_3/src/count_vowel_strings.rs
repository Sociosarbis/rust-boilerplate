use super::*;

impl Solution {
  pub fn count_vowel_strings(n: i32) -> i32 {
    let mut dp = vec![vec![0;5];2];
    for i in 0..5 {
      dp[0][i] = 1;
    }
    for i in 1..n as usize {
      for j in 0..5 {
        dp[i & 1][j] = 0;
        for k in 0..=j {
          dp[i & 1][j] += dp[(i - 1) & 1][k];
        }
      }
    }
    dp[(n as usize - 1) & 1].iter().fold(0, |acc, &item| acc + item)
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
  fn test_count_vowel_strings_simple() {
    let suites = vec![
      Suite {
        n: 1,
        ret: 5,
      },
      Suite {
        n: 2,
        ret: 15,
      },
      Suite {
        n: 33,
        ret: 66045,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_vowel_strings(s.n));
    }
  }
}