use super::*;

impl Solution {
  pub fn alternate_digit_sum(mut n: i32) -> i32 {
    let mut c = 0;
    let mut ret = 0;
    while n != 0 {
      c += 1;
      ret += if c & 1 == 1 { n % 10 } else { -(n % 10) };
      n /= 10;
    }

    if c & 1 == 1 {
      ret
    } else {
      -ret
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32,
  }

  #[test]
  fn test_alternate_digit_sum_simple() {
    let suites = vec![
      Suite { n: 521, ret: 4 },
      Suite { n: 111, ret: 1 },
      Suite { n: 886996, ret: 0 },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::alternate_digit_sum(s.n));
    }
  }
}
