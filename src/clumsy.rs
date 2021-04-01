use super::Solution;

impl Solution {
  pub fn clumsy(mut n: i32) -> i32 {
      let mut op = 0;
      let mut ret = 0;
      let mut factor = 1;
      while n > 0 {
        let mut temp = n;
        if op == 0 {
          n -= 1;
          if n != 0 {
            temp *= n;
            n -= 1;
            if n != 0 {
              temp /= n;
            }
          }
          ret += temp * factor;
          if factor == 1 {
            factor = -1;
          }
        } else {
          ret += temp;
        }
        n -= 1;
        op = 1 - op;
      }
      ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: i32,
    ret: i32
  }

  #[test]
  fn test_clumsy_simple() {
    let suites = vec![
      Suite {
        s: 4,
        ret: 7
      },
      Suite {
        s: 10,
        ret: 12
      }
    ];

    for s in suites {
      assert_eq!(Solution::clumsy(s.s), s.ret);
    }
  }
}