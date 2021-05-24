use super::Solution;

impl Solution {
  pub fn strange_printer(s: String) -> i32 {
    let target: Vec<char> = s.chars().collect();
    let mut dp = vec![vec![0;s.len()];s.len()];
    for j in 0..target.len() {
      // 为了无后效性，i 和 j 的距离需要从小递增
      for i in (0..j+1).rev() {
        if i == j {
          dp[i][j] = 1;
        } else {
          // 因为首字符总需要一次打印并将首字符在初次打到整个区间不影响最小打印次数，所以当首尾字符相同时
          // 尾字符可以在同一次打印中顺带打印出来，所以有如下的转移方程
          if target[i] == target[j] {
            dp[i][j] = dp[i][j - 1];
          } else {
            let mut min = -1;
            for k in i..j {
              let sum = dp[i][k] + dp[k + 1][j];
              if sum < min || min == -1 {
                min = sum;
              }
            }
            dp[i][j] = min;
          }
        }
      }
    }
    dp[0][target.len() - 1]
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
  fn test_strange_printer_simple() {
    let suites = vec![
      Suite {
        s: "aaabbb",
        ret: 2,
      },
      Suite {
        s: "aba",
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(Solution::strange_printer(s.s.to_owned()), s.ret);
    }
  }
}