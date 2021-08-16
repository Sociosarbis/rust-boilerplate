use super::Solution;
use std::collections::HashMap;

impl Solution {
  pub fn count_arrangement(n: i32) -> i32 {
    let mut num_to_options = vec![vec![];n as usize + 1];
    for i in 1..num_to_options.len() {
      let mut factor = 1;
      while i * factor <= n as usize {
        let res = i * factor;
        num_to_options[i].push(res);
        if factor > 1 {
          num_to_options[res as usize].push(i);
        }
        factor += 1;
      }
    }
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(0, 1);
    for i in 1..num_to_options.len() {
      let mut new_dp: HashMap<i32, i32> = HashMap::new();
      for (k, v) in &dp {
        for j in &num_to_options[i] {
          if k & (1 << j) == 0 {
            let key = k | (1 << j);
            if !new_dp.contains_key(&key) {
              new_dp.insert(key, 0);
            }
            *new_dp.get_mut(&key).unwrap() += v;
          }
        }
      }
      dp = new_dp;
    }
    dp.iter().fold(0, |acc, item| { acc + item.1 })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_count_arrangement_simple() {
    let suites = vec![
      Suite {
        n: 2,
        ret: 2
      },
      Suite {
        n: 1,
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_arrangement(s.n));
    }
  }
}