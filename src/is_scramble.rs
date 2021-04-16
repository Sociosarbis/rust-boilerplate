use super::Solution;

impl Solution {
  pub fn is_scramble(s1: String, s2: String) -> bool {
    if s1 == s2 {
      return true;
    }
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let mut dp: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;chars1.len()];chars1.len()];chars1.len()];
    Solution::is_scramble_dfs(&chars1, &chars2, 0, 0, chars1.len(), &mut dp)
  }

  fn is_scramble_dfs(chars1: &Vec<char>, chars2: &Vec<char>, i1: usize, i2: usize, len: usize, dp: &mut Vec<Vec<Vec<u8>>>) -> bool {
    if dp[i1][i2][len - 1] != 0 {
      return if dp[i1][i2][len - 1] == 1 { true } else { false }
    }
    let is_equal = {
      let mut ret = true;
      for i in 0..len {
        if chars1[i1 + i] != chars2[i2 + i] {
          ret = false;
          break;
        }
      }
      ret
    };
    if is_equal {
      return true;
    }

    let mut head = vec![0;26];
    let mut count1 = 0;
    let mut tail = vec![0;26];
    let mut count2 = 0;
    for i in 0..len {
      let mut index = chars2[i2 + i] as usize - 97;
      head[index] += 1;
      if head[index] == 1 {
        count1 += 1;
      } else if head[index] == 0 {
        count1 -= 1;
      }
      index = chars2[i2 + len - 1 - i] as usize - 97;
      tail[index] += 1;
      if tail[index] == 1 {
        count2 += 1;
      } else if tail[index] == 0 {
        count2 -= 1;
      }
      index = chars1[i1 + i] as usize - 97;
      head[index] -= 1;
      if head[index] == -1 {
        count1 += 1;
      } else if head[index] == 0 {
        count1 -= 1;
      }

      tail[index] -= 1;
      if tail[index] == -1 {
        count2 += 1;
      } else if tail[index] == 0 {
        count2 -= 1;
      }

      if count1 == 0 && i != len - 1 {
        if Solution::is_scramble_dfs(chars1, chars2, i1, i2, i + 1, dp) && Solution::is_scramble_dfs(chars1, chars2, i1 + i + 1, i2 + i + 1, len - i - 1, dp) {
          dp[i1][i2][len - 1] = 1; 
          return true
        }
      }

      if count2 == 0 && i != len - 1 {
        if Solution::is_scramble_dfs(chars1, chars2, i1, i2 + len - 1 - i, i + 1, dp) && Solution::is_scramble_dfs(chars1, chars2, i1 + i + 1, i2, len - i - 1, dp) {
          dp[i1][i2][len - 1] = 1;
          return true
        }
      }
    }
    dp[i1][i2][len - 1] = 2;
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s1: String,
    s2: String,
    ret: bool
  }

  #[test]
  fn test_is_scramble_simple() {
    let suites = vec![
      Suite {
        s1: "great".to_owned(),
        s2: "rgeat".to_owned(),
        ret: true
      },
      Suite {
        s1: "abcde".to_owned(),
        s2: "caebd".to_owned(),
        ret: false
      },
      Suite {
        s1: "a".to_owned(),
        s2: "a".to_owned(),
        ret: true
      },
      Suite {
        s1: "abcdbdacbdac".to_owned(),
        s2: "bdacabcdbdac".to_owned(),
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::is_scramble(s.s1, s.s2), s.ret);
    }
  }
}