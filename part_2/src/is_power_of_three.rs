use super::*;

impl Solution {
  pub fn is_power_of_three(mut n: i32) -> bool {
    if n < 1 {
      return false;
    }
    while n != 1 {
      if n % 3 != 0 {
        return false;
      }
      n /= 3;
    }
    true
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
  fn test_is_power_of_three_simple() {
    let suites = vec![
      Suite {
        n: 27,
        ret: true
      },
      Suite {
        n: 0,
        ret: false
      },
      Suite {
        n: 9,
        ret: true
      },
      Suite {
        n: 45,
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_power_of_three(s.n));
    }
  }
}