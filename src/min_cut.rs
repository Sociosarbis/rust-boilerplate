use super::Solution;


impl Solution {
  pub fn min_cut(s: String) -> i32 {
    if s.is_empty() { return 0; }
    let mut cut = vec![0;s.len()];
    let mut dp = vec![vec![false;s.len()];s.len()];
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
      cut[i] = i;
      for j in (0..i + 1).rev() {
        // 判断[j..i+1]能不能形成回文串
        if chars[i] == chars[j] && (i - j <= 1 || dp[j + 1][i - 1]) {
          dp[j][i] = true;
          if j == 0 {
            cut[i] = 0
          } else if cut[j - 1] + 1 < cut[i] {
            cut[i] = cut[j - 1] + 1;
          }
        }
      }
    }
    cut[s.len() - 1] as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: i32,
  }

  #[test]
  fn min_cut_simple() {
    let suites = vec![
      Suite {
        s: "aab".to_string(),
        ret: 1
      },
      Suite {
        s: "a".to_string(),
        ret: 0
      },
      Suite {
        s: "ab".to_string(),
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(Solution::min_cut(s.s), s.ret);
    }
  }
}