use super::Solution;

impl Solution {
  pub fn longest_palindrome_subseq(s: String) -> i32 {
    if s.len() == 1 {
      return 1;
    }
    let mut ret = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut dp: Vec<Vec<i32>> = vec![vec![0;s.len()];s.len()];
    for i in 0..chars.len() {
      for j in (i..chars.len()).rev() {
        let mut temp = 0;
        if chars[i] == chars[j] {
          temp = if i == j { 1 } else { 2 };
          if i > 0 && j + 1 < chars.len() {
            temp += dp[i - 1][j + 1];
          }
        } else {
          if i > 0 {
            temp = dp[i - 1][j];
          }
          if j + 1 < chars.len() && temp < dp[i][j + 1] {
            temp = dp[i][j + 1];
          }
        }
        dp[i][j] = temp;
        if temp > ret {
          ret = temp;
        }
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: i32
  }

  #[test]
  fn test_longest_palindrome_subseq_simple() {
    let suites = vec![
      Suite {
        s: "bbbab",
        ret: 4
      },
      Suite {
        s: "cbbd",
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::longest_palindrome_subseq(s.s.to_owned()));
    }
  }
}