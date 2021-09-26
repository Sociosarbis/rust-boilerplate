use super::*;

impl Solution {
  pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
    // 将1分为共有和独有两类
    let mut xor = a ^ b;
    // 如果将共有的1相加，等于 1 << 1
    let mut and = (a & b) << 1;
    // 循环直到没有交叉的1
    while and != 0 {
      a = xor;
      b = and;
      xor = a ^ b;
      and = (a & b) << 1;
    }
    xor
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
  fn test_get_sum_simple() {
    let suites = vec![
      Suite {
        a: 1,
        b: 2,
        ret: 3
      },
      Suite {
        a: 2,
        b: 3,
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::get_sum(s.a, s.b));
    }
  }
}