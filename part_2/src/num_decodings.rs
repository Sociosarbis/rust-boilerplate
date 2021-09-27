use super::*;

impl Solution {
  pub fn num_decodings(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    if chars[0] == '0' {
      return 0;
    }
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0;10];2];2];
    if chars[0] == '*' {
      for i in 1..10 {
        dp[0][0][i] = 1;
      }
    } else {
      dp[0][0][chars[0] as usize - 48] = 1;
    }
    for i in 1..chars.len() {
      let index = i & 1;
      let prev_index = 1 - index;
      let num = if chars[i] == '*' { 0 } else { chars[i] as usize - 48 };
      let prev_num = if chars[i - 1] == '*' { 0 } else { chars[i - 1] as usize - 48 };
      if chars[i] == '*' {
        if chars[i - 1] == '*' {
          for i in 1..10 {
            dp[index][1][i] = dp[prev_index][0][1];
          }
          for i in 1..7 {
            dp[index][1][i] += dp[prev_index][0][2];
          }
        } else {
          if chars[i - 1] == '1' {
            for i in 1..10 {
              dp[index][1][i] = dp[prev_index][0][1];
            }
          } else if chars[i - 1] == '2' {
            for i in 1..7 {
              dp[index][1][i] = dp[prev_index][0][2];
            }
          } else {
            for i in 1..10 {
              dp[index][1][i] = 0;
            }
          }
        }
      } else {
        dp[index][1][num] = {
          if chars[i - 1] == '*' {
            dp[prev_index][0][1] + dp[prev_index][0][2] * if num > 6 { 0 } else { 1 }
          } else if chars[i - 1] == '1' {
              dp[prev_index][0][1]
            } else if chars[i - 1] == '2' {
              dp[prev_index][0][2] * if num > 6 { 0 } else { 1 }
            } else {
              0
            }
        } % 1000000007;
      }

      if chars[i] == '*' {
        for j in 1..10 {
          dp[index][0][j] = if chars[i - 1] != '*' {
            dp[prev_index][0][prev_num] + dp[prev_index][1][prev_num]
          } else {
            let mut temp = 0;
            for k in 1..10 {
              temp += dp[prev_index][0][k] + dp[prev_index][1][k];
            }
            temp
          } % 1000000007;
        }
      } else if chars[i] == '0' {
        dp[index][0][0] = 0;
      } else {
        dp[index][0][num] = if chars[i - 1] != '*' {
          dp[prev_index][0][prev_num] + dp[prev_index][1][prev_num]
        } else {
          let mut temp = 0;
          for j in 1..10 {
            temp += dp[prev_index][0][j] + dp[prev_index][1][j];
          }
          temp
        } % 1000000007;
      }
    }
    let last_char = *chars.last().unwrap();
    let last_index = (chars.len() - 1) & 1;
    let mut ret = 0;
    if last_char == '*' {
      for i in 0..2 {
       for j in 1..10 {
         ret = (ret + dp[last_index][i][j]) % 1000000007
       }
      }
    } else {
      let last_num = last_char as usize - 48;
      for i in 0..2 {
        ret = (ret + dp[last_index][i][last_num]) % 1000000007
      }
    }
    ret as i32
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
  fn test_num_encodings_simple() {
    let suites = vec![
      Suite {
        s: "*",
        ret: 9
      },
      Suite {
        s: "1*",
        ret: 18
      },
      Suite {
        s: "2*",
        ret: 15
      },
      Suite {
        s: "*1*1*0",
        ret: 404
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_decodings(s.s.to_owned()));
    }
  }
}