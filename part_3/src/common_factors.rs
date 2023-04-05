use super::*;

impl Solution {
  pub fn common_factors(a: i32, b: i32) -> i32 {
    let c = Solution::get_greatest_common_divisor(a, b);
    let mut ret = 1;
    for i in 1..c {
      if a % i == 0 && b % i == 0 {
        ret += 1;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: i32,
    b: i32,
    ret: i32
  }

  #[test]
  fn test_common_factors_simple() {
    let suites = vec![
      Suite {
        a: 12,
        b: 6,
        ret: 4
      },
      Suite {
        a: 25,
        b: 30,
        ret: 2
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::common_factors(s.a, s.b));
    }
  }
}