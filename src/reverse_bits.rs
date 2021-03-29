use super::Solution;

impl Solution {
  pub fn reverse_bits(mut x: u32) -> u32 {
    let mut ret = 0;
    for i in 0..32 {
      if x & 1 != 0 {
        ret |= 1 << (31 - i);
      }
      x >>= 1;
      if x == 0 {
        break;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    x: u32,
    ret: u32
  }

  #[test]
  fn test_reverse_bits_simple() {
    let suites = vec![
      Suite {
        x: 43261596,
        ret: 964176192 
      },
      Suite {
        x: 4294967293,
        ret: 3221225471
      }
    ];

    for s in suites {
      assert_eq!(Solution::reverse_bits(s.x), s.ret);
    }
  }
}




