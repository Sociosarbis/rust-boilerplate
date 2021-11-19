use super::*;

impl Solution {
  pub fn integer_replacement(num: i32) -> i32 {
    let mut n = num as i64;
    let mut ret = 0;
    while n != 1 {
      if n & 1 == 0 {
        n >>= 1;
      } else {
        if (n >> 1) & 1 == 1 {
          n += 1;
        } else {
          n -= 1;
        }
      }
      ret += 1;
    }
    ret
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
  fn test_integer_replacement_simple() {
    let suites = vec![
      Suite {
        n: 8,
        ret: 3
      },
      Suite {
        n: 7,
        ret: 4
      },
      Suite {
        n: 4,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::integer_replacement(s.n));
    }
  }
}