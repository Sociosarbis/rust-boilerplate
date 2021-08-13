use super::Solution;


impl Solution {
  pub fn count_digit_one(n: i32) -> i32 {
    if n == 0 {
      return 0;
    }
    let mut dp: Vec<i32> = vec![0;n as usize + 1];
    let mut base: i32 = 1;
    dp[1] = 1;
    for i in 2..n + 1 {
      if i == base * 10 {
        base *= 10;
      }
      dp[i as usize] = dp[i as usize % base as usize] + if i / base == 1 { 1 } else { 0 };
    }
    dp.iter().fold(0, |acc, item| { acc + item })
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }
  #[test]
  fn test_count_digit_one_simple() {
    let suites = vec![
      Suite {
        n: 13,
        ret: 6
      },
      Suite {
        n: 0,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_digit_one(s.n));
    }
  }
}