use super::*;

impl Solution {
  pub fn count_digits(num: i32) -> i32 {
    let mut ret = 0;
    let mut temp = num;
    while temp != 0 {
      if num % (temp % 10) == 0 {
        ret += 1;
      }
      temp /= 10;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: i32,
  }

  #[test]
  fn test_count_digits_simple() {
    let suites = vec![Suite { num: 7, ret: 1 }, Suite { num: 121, ret: 2 }];

    for s in suites {
      assert_eq!(s.ret, Solution::count_digits(s.num));
    }
  }
}
