use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut x_base = 1;
    let mut ret = HashSet::new();
    while x_base + 1 <= bound {
      let mut y_base = 1;
      while x_base + y_base <= bound {
        ret.insert(x_base + y_base);
        if y != 1 {
          y_base *= y;
        } else {
          break;
        }
      }
      if x != 1 {
        x_base *= x;
      } else {
        break;
      }
    }
    ret.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    x: i32,
    y: i32,
    bound: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_powerful_integers_simple() {
    let suites = vec![
      Suite {
        x: 2,
        y: 3,
        bound: 10,
        ret: vec![2,3,4,5,7,9,10]
      },
      Suite {
        x: 3,
        y: 5,
        bound: 15,
        ret: vec![2,4,6,8,10,14]
      },
    ];

    for s in suites {
      let mut ret = Solution::powerful_integers(s.x, s.y, s.bound);
      ret.sort_unstable();
      assert_eq!(s.ret, ret);
    }
  }
}