use super::Solution;

impl Solution {
  pub fn hamming_weight (mut n: u32) -> i32 {
      let mut ret = 0;
      while n != 0 {
        // 与 n - 1的&运算会让n 1的数量减少1
        n &= n - 1;
        ret += 1;
      }
      ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: u32,
    ret: i32
  }

  #[test]
  fn hamming_weight_simple() {
    let suites = vec![
      Suite {
        n: 11,
        ret: 3
      },
      Suite {
        n: 128,
        ret: 1
      },
      Suite {
        n: 4294967293,
        ret: 31
      }
    ];

    for s in suites {
      assert_eq!(Solution::hamming_weight(s.n), s.ret);
    }
  }
}