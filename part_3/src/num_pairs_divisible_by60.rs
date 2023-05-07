use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut dp: HashMap<i32, i32> = HashMap::new();
    let mut ret = 0;
    for t in time {
      let b = t % 60;
      let a = if b == 0 { 0 } else { 60 - b };
      if let Some(&c) = dp.get(&a) {
        ret += c;
      }
      if let Some(c) = dp.get_mut(&b) {
        *c += 1;
      } else {
        dp.insert(b, 1);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    time: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_num_pairs_divisible_by60_simple() {
    let suites = vec![
      Suite {
        time: vec![30,20,150,100,40],
        ret: 3
      },
      Suite {
        time: vec![60,60,60],
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_pairs_divisible_by60(s.time));
    }
  }
}