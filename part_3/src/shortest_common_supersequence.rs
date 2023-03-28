use super::*;

impl Solution {
  pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    if str1 == str2 {
        return str1;
    }
    let mut max_index = (0, 0);
    let mut max = 0;
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let mut dp = vec![vec![0;str2.len() + 1];str1.len()+ 1];
    // 寻找最大公共序列的经典方法
    for i in 1..=str1.len() {
      for j in 1..=str2.len() {
        if chars1[i - 1] == chars2[j - 1] {
          dp[i][j] = dp[i - 1][j - 1] + 1;
          if dp[i][j] > max {
            max = dp[i][j];
            max_index = (i, j);
          }
        } else {
          dp[i][j] = if dp[i - 1][j] > dp[i][j - 1] { dp[i - 1][j] } else { dp[i][j - 1] };
        }
      }
    }
    if max == 0 {
      return format!("{}{}", str1, str2);
    }
    let mut ret: Vec<char> = vec![];
    let mut prev_index = (chars1.len(), chars2.len());
    while max != 0 {
      let (i, j) = max_index;
      if dp[i - 1][j - 1] + 1 == dp[i][j] && chars1[i - 1] == chars2[j - 1] {
        for k in (j..prev_index.1).rev() {
          ret.push(chars2[k]);
        }
        for k in (i..prev_index.0).rev() {
          ret.push(chars1[k]);
        }
        ret.push(chars1[i - 1]);
        max_index = (i - 1, j - 1);
        prev_index = max_index;
        max -= 1;
      } else {
        if dp[i - 1][j] == dp[i][j] {
          max_index = (i - 1, j);
        } else {
          max_index = (i, j - 1);
        }
      }
    }
    for i in (0..max_index.1).rev() {
      ret.push(chars2[i]);
    }
    for i in (0..max_index.0).rev() {
      ret.push(chars1[i]);
    }
    ret.iter().rev().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    str1: String,
    str2: String,
    ret: String
  }

  #[test]
  fn test_shortest_common_supersequence_simple() {
    let suites = vec![
      Suite {
        str1: "abac".to_string(),
        str2: "cab".to_string(),
        ret: "cabac".to_string(),
      },
      Suite {
        str1: "aaaaaaaa".to_string(),
        str2: "aaaaaaaa".to_string(),
        ret: "aaaaaaaa".to_string()
      },
      Suite {
        str1: "bbbaaaba".to_string(),
        str2: "bbababbb".to_string(),
        ret: "bbabaaababb".to_string()
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::shortest_common_supersequence(s.str1, s.str2));
    }
  }
}