use super::Solution;

impl Solution {
  pub fn num_decodings(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut dp: Vec<i32> = vec![0;chars.len()];
    if chars[0] == '0' {
      return 0;
    } else {
      dp[0] = 1;
    }
    for i in 1..chars.len() {
      let num: String = chars[i - 1..i+1].iter().collect();
      if chars[i] != '0' {
        dp[i] = dp[i - 1];
        if let Ok(res) = num.parse::<i32>() {
          if res > 9 && res < 27 {
            if i > 1 {
              dp[i] += dp[i - 2];
            } else {
              dp[i] += 1
            }
          }
        }
      } else {
        if let Ok(res) = num.parse::<i32>() {
          if res > 9 && res < 27 {
            if i > 1 {
              dp[i] = dp[i - 2];
            } else {
              dp[i] = 1;
            }
            continue;
          }
        }
        return 0;
      }
    }
    dp[chars.len() - 1]
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
  fn test_num_decodings_simple() {
    let suites = vec![
      Suite {
        s: "12",
        ret: 2,
      },
      Suite {
        s: "226",
        ret: 3
      },
      Suite {
        s: "0",
        ret: 0
      },
      Suite {
        s: "10",
        ret: 1,
      },
      Suite {
        s: "2101",
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_decodings(s.s.to_owned()), s.ret);
    }
  }
}