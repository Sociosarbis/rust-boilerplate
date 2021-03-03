use super::Solution;

impl Solution {
  pub fn count_bits(num: i32) -> Vec<i32> {
      let mut ret = vec![0];
      let mut base = 1;
      let mut prev_base = 0;
      for i in 1..num + 1 {
        if i == base {
          ret.push(1);
          prev_base = base;
          base <<= 1;
        } else {
          ret.push(1 + ret[(i - prev_base) as usize]);
        }
      }
      ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    num: i32,
    ret: Vec<i32>
  }

  #[test]
  fn count_bits_simple() {
    let suites = vec![
      Suite {
        num: 2,
        ret: vec![0, 1, 1]
      },
      Suite {
        num: 5,
        ret: vec![0,1,1,2,1,2]
      }
    ];
    

    for s in suites {
      assert_eq!(Solution::count_bits(s.num), s.ret);
    }
  }
}