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
      let c = chars[i];
      let prev_c = chars[i - 1];
      let num = if c == '*' { 10 } else { c as usize - 48 };
      let prev_num = if prev_c == '*' { 10 } else { prev_c as usize - 48 };
      match c {
        '*' => {
          match prev_c {
            '*' => {
              for i in 1..10 {
                dp[index][1][i] = dp[prev_index][0][1];
              }
              for i in 1..7 {
                dp[index][1][i] += dp[prev_index][0][2];
              }
            }
            '1' => {
              for i in 1..10 {
                dp[index][1][i] = dp[prev_index][0][1];
              }
            }
            '2' => {
              for i in 1..7 {
                dp[index][1][i] = dp[prev_index][0][2];
              }
              for i in 7..10 {
                dp[index][1][i] = 0;
              }
            }
            _ => {
              for i in 1..10 {
                dp[index][1][i] = 0;
              }
            }
          }
        }
        _ => {
          dp[index][1][num] = (match prev_c {
            '*' | '1' => { dp[prev_index][0][1] }
            _ => { 0 }
          } + match prev_c {
            '*' | '2' => { dp[prev_index][0][2] * if num > 6 { 0 } else { 1 } }
            _ => { 0 }
          }) % 1000000007;
        }
      }

      match c {
        '*' => {
          for j in 1..10 {
            dp[index][0][j] = if prev_c != '*' {
              dp[prev_index][0][prev_num] + dp[prev_index][1][prev_num]
            } else {
              (1..10).fold(0, |acc, k| acc + dp[prev_index][0][k] + dp[prev_index][1][k])
            } % 1000000007;
          }
        }
        '0' => { dp[index][0][0] = 0; }
        _ => {
          dp[index][0][num] = if prev_c != '*' {
            dp[prev_index][0][prev_num] + dp[prev_index][1][prev_num]
          } else {
            (1..10).fold(0, |acc, j| acc + dp[prev_index][0][j] + dp[prev_index][1][j])
          } % 1000000007;
        }
      }
    }
    let last_char = *chars.last().unwrap();
    let last_index = (chars.len() - 1) & 1;
    (if last_char == '*' {
      (0..2).fold(0, |acc1, i| 
        (acc1 + (1..10).fold(0, |acc2, j| 
          (acc2 + dp[last_index][i][j]) % 1000000007
        )) % 1000000007
      )
    } else {
      let last_num = last_char as usize - 48;
      (0..2).fold(0, |acc, i| (acc + dp[last_index][i][last_num]) % 1000000007)
    }) as i32
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