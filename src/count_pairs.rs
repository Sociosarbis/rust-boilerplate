use super::Solution;

use std::collections::HashMap;

impl Solution {
  pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mask = 1000000007;
    let mut max = 0;
    let mut num_to_count: HashMap<i32, i32> = HashMap::new();
    let mut ret = 0;
    let mut unique = vec![];
    for d in deliciousness {
      if !num_to_count.contains_key(&d) {
        num_to_count.insert(d, 0);
        unique.push(d);
        if d > max {
          max = d;
        }
      }
      *num_to_count.get_mut(&d).unwrap() += 1;
    }
    unique.sort();
    for i in 0..unique.len() {
      let a = unique[i];
      let c = *num_to_count.get(&a).unwrap() as i64;
      let mut sum = 1;
      while sum < a * 2 {
        sum <<= 1;
      }
      while sum - a <= max {
        let target = sum - a;
        if let Some(&n) = num_to_count.get(&target) {
          ret += if target == a { n as i64 * (n as i64 - 1) / 2  } else { c * n as i64 } % mask;
          if ret > mask {
            ret %= mask;
          }
        }
        sum <<= 1;
      }
    }
    ret as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    deliciousness: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_count_pairs_simple() {
    let suites = vec![
      Suite {
        deliciousness: vec![1,3,5,7,9],
        ret: 4
      },
      Suite {
        deliciousness: vec![1,1,1,3,3,3,7],
        ret: 15
      }
    ];
    for s in suites {
      assert_eq!(s.ret, Solution::count_pairs(s.deliciousness));
    }
  }
}