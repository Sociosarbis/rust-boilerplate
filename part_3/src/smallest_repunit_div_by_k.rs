use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut start = 1;
    let mut ret = 1;
    while start < k {
      start = start * 10 + 1;
      ret += 1;
    }
    start %= k;
    if start == 0 {
      return ret;
    }
    let mut base = start;
    let mut visited = HashSet::new();
    visited.insert(base);
    loop {
      base = ((base * 10) % k + 1) % k;
      ret += 1;
      if base == 0 {
        break;
      }
      if visited.contains(&base) {
        return -1;
      }
      visited.insert(base);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    k: i32,
    ret: i32
  }

  #[test]
  fn test_smallest_repunit_div_by_k_simple() {
    let suites = vec![
      Suite {
        k: 1,
        ret: 1
      },
      Suite {
        k: 2,
        ret: -1
      },
      Suite {
        k: 3,
        ret: 3
      },
      Suite {
        k: 8,
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::smallest_repunit_div_by_k(s.k));
    }
  }
}