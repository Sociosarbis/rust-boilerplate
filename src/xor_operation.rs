use super::Solution;

impl Solution {
  pub fn xor_operation(n: i32, start: i32) -> i32 {
    let mut ret = start;
    let mut cur = start;
    for _ in 1..n {
      cur += 2;
      ret ^= cur;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    start: i32,
    ret: i32
  }

  #[test]
  fn test_xor_operation_simple() {
    let suites = vec![
      Suite {
        n: 5,
        start: 0,
        ret: 8
      },
      Suite {
        n: 4,
        start: 3,
        ret: 8
      },
      Suite {
        n: 1,
        start: 7,
        ret: 7
      }
    ];

    for s in suites {
      assert_eq!(Solution::xor_operation(s.n, s.start), s.ret);
    }
  }
}