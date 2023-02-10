use super::*;
use std::collections::HashMap;

impl Solution {
  pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(0, 1);
    for _ in 0..n {
      let mut next_dp = HashMap::new();
      for (k, c) in dp {
        for i in 0..6 {
          let count = (k >> (4 * i)) & 15;
          if count + 1 <= roll_max[i] {
            let next_k = (count + 1) << (4 * i);
            if let Some(v) = next_dp.get_mut(&next_k) {
              *v = (*v  + c) % 1000000007;
            } else {
              next_dp.insert(next_k, c);
            }
          }
        }
      } 
      dp = next_dp;
    }
    let mut ret = 0;
    for (_, v) in dp {
      ret = (ret + v) % 1000000007;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    roll_max: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_die_simulator_simple() {
    let suites = vec![
      Suite {
        n: 2,
        roll_max: vec![1,1,2,2,2,3],
        ret: 34
      },
      Suite {
        n: 2,
        roll_max: vec![1,1,1,1,1,1],
        ret: 30
      },
      Suite {
        n: 3,
        roll_max: vec![1,1,1,2,2,3],
        ret: 181
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::die_simulator(s.n, s.roll_max));
    }
  }
}