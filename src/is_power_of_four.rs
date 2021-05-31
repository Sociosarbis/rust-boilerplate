use super::Solution;

impl Solution {
  pub fn is_power_of_four(n: i32) -> bool {
    if n <= 0 || (n & (n - 1)) != 0 {
      return false;
    }
    (n & 715827882) == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: bool
  }

  #[test]
  fn test_is_power_of_four_simple() {
    let suites = vec![
      Suite {
        n: 16,
        ret: true
      },
      Suite {
        n: 5,
        ret: false
      },
      Suite {
        n: 1,
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::is_power_of_four(s.n), s.ret);
    }
  }
}