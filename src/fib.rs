use super::Solution;

impl Solution {
  pub fn fib(n: i32) -> i32 {
    if n == 0 { return 0 }
    if n == 1 { return 1 }
    let mut roll = [0, 1];
    for _ in 2..n as usize {
      let tmp = roll[0] + roll[1];
      roll[0] = roll[1];
      roll[1] = tmp;
    }
    return roll[0] + roll[1];
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
  fn fib_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        n: 2,
        ret: 1
      },
      Suite {
        n: 3,
        ret: 2
      },
      Suite {
        n: 4,
        ret: 3
      }
    ];
    for s in suites {
      assert_eq!(Solution::fib(s.n), s.ret);
    }
  }
}