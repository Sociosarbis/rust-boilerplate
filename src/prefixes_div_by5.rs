use super::Solution;

impl Solution {
  pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut ret: Vec<bool> = vec![false;a.len()];
    let mut prev = 0;
    for i in 0..a.len() {
      prev = ((prev << 1) + a[i]) % 5;
      if prev == 0 {
        ret[i] = true;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: Vec<i32>,
    ret: Vec<bool>
  }

  #[test]
  fn prefixes_div_by5_simple() {
    let suites = vec![
      Suite {
        a: vec![0,1,1,1,1,1],
        ret: vec![true,false,false,false,true,false]
      },
      Suite {
        a: vec![1,1,1,0,1],
        ret: vec![false,false,false,false,false]
      }
    ];

    for s in suites {
      assert_eq!(Solution::prefixes_div_by5(s.a), s.ret);
    }
  }
}