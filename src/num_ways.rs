use super::Solution;

impl Solution {
  pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
    if steps == 1 || arr_len == 1 {
      return 1;
    }
    let mut dp = vec![vec![0;arr_len as usize]; 2];
    dp[0][0] = 1;
    for i in 0..steps as usize {
      let index = i % 2; 
      for j in 0..arr_len as usize {
        let num = dp[index][j];
        if num != 0 {
          dp[1 - index][j] = Solution::safe_add(dp[1 - index][j], num);
          if j < arr_len as usize - 1 {
            dp[1 - index][j + 1] = Solution::safe_add(dp[1 - index][j + 1], num);
          }
          if j > 0 {
            dp[1 - index][j - 1] = Solution::safe_add(dp[1 - index][j - 1], num);
          }
          dp[index][j] = 0;
        }
      }
    }
    dp[steps as usize % 2][0]
  }

  fn safe_add(a: i32, b: i32) -> i32 {
    let sum = if a < 1000000007 { a } else { a % 1000000007 } + if b < 1000000007 { b } else { b % 1000000007 };
    if sum < 1000000007 { sum } else { sum % 1000000007 }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    steps: i32,
    arr_len: i32,
    ret: i32
  }

  #[test]
  fn test_num_ways_simple() {
    let suites = vec![
      Suite {
        steps: 2,
        arr_len: 4,
        ret: 2
      },
      Suite {
        steps: 4,
        arr_len: 2,
        ret: 8
      }
    ];

    for s in suites {
      assert_eq!(Solution::num_ways(s.steps, s.arr_len), s.ret);
    }
  }
}