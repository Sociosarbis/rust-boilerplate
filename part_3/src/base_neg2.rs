use super::*;

impl Solution {
  pub fn base_neg2(mut n: i32) -> String {
    if n == 0 {
      return "0".to_string();
    }
    let mut ret = String::new();
    while n != 0 {
      if n & 1 == 1 {
        n -= 1;
        ret.push('1');
      } else {
        ret.push('0');
      }
      n /= -2;
    }
    ret.chars().into_iter().rev().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: String
  }

  #[test]
  fn test_base_neg2_simple() {
    let suites = vec![
      Suite {
        n: 2,
        ret: "110".to_string()
      },
      Suite {
        n: 3,
        ret: "111".to_string()
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::base_neg2(s.n));
    }
  }
 }