use super::*;

impl Solution {
  pub fn pivot_integer(n: i32) -> i32 {
    let mut temp = n * n + n;
    if temp & 1 == 0 {
      temp >>= 1;
      let mut l = 1;
      let mut r = n;
      while l <= r {
        let mid = (l + r) >> 1;
        let mid_square = mid * mid;
        if mid_square > temp {
          r = mid - 1;
        } else if mid_square < temp {
          l = mid + 1;
        } else {
          return mid;
        }
      }
    }
    -1
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
  fn test_pivot_integer_simple() {
    let suites = vec![
      Suite { n: 8, ret: 6 },
      Suite { n: 1, ret: 1 },
      Suite { n: 4, ret: -1 },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::pivot_integer(s.n));
    }
  }
}
