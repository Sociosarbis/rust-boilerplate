use super::Solution;

impl Solution {
  pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut res = x ^ y;
    let mut ret = 0;
    while res != 0 {
      res &= res - 1;
      ret += 1;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    x: i32,
    y: i32,
    ret: i32
  }

  #[test]
  fn test_hamming_distance_simple() {
    let suites = vec![
      Suite {
        x: 1,
        y: 4,
        ret: 2
      },
      Suite {
        x: 3,
        y: 1,
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(Solution::hamming_distance(s.x, s.y), s.ret);
    }
  }
}
