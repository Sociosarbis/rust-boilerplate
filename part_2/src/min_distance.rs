use super::*;

impl Solution {
  pub fn min_distance(word1: String, word2: String) -> i32 {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let mut dp: Vec<Vec<i32>> = vec![vec![0;word2.len() + 1];word1.len() + 1];
    for i in 0..word2.len() + 1 {
      dp[0][i] = i as i32;
    }
    for i in 0..word1.len() + 1 {
      dp[i][0] = i as i32;
    }
    for i in 1..word1.len() + 1 {
      for j in 1..word2.len() + 1 {
        dp[i][j] = if chars1[i - 1] == chars2[j - 1] { 
          dp[i - 1][j - 1] 
        } else {
          if dp[i][j - 1] > dp[i - 1][j] {
            dp[i - 1][j] + 1
          } else {
            dp[i][j - 1] + 1
          }
        }
      }
    }
    dp[word1.len()][word2.len()]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    word1: &'a str,
    word2: &'a str,
    ret: i32
  }

  #[test]
  fn test_min_distance_simple() {
    let suites = vec![
      Suite {
        word1: "sea",
        word2: "eat",
        ret: 2
      },
      Suite {
        word1: "leetcode",
        word2: "etco",
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_distance(s.word1.to_owned(), s.word2.to_owned()));
    }
  }
}