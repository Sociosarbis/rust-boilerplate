use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let mut dp = HashMap::new();
    dp.insert(0, 1);
    for _ in 0..n {
      let mut next_dp = HashMap::new();
      for i in 1..=k {
        for (v, &c) in &dp {
          let next_v = v + i;
          if next_v <= target {
            if let Some(next_c) = next_dp.get_mut(&next_v) {
              *next_c = (*next_c + c) % 1000000007
            } else {
              next_dp.insert(next_v, c);
            }
          }
        }
      }
      dp = next_dp;
    }
    dp.remove(&target).unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    k: i32,
    target: i32,
    ret: i32,
  }

  #[test]
  fn test_num_rolls_to_target_simple() {
    let suites = vec![
      Suite {
        n: 1,
        k: 6,
        target: 3,
        ret: 1,
      },
      Suite {
        n: 2,
        k: 6,
        target: 7,
        ret: 6,
      },
      Suite {
        n: 30,
        k: 30,
        target: 500,
        ret: 222616187,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_rolls_to_target(s.n, s.k, s.target));
    }
  }
}
