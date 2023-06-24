use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn get_max_grid_happiness(
    m: i32,
    n: i32,
    introverts_count: i32,
    extroverts_count: i32,
  ) -> i32 {
    // 逐行作动态规划，比起以所有格子作为每步状态的做法，可能的状态更少
    let mut dp: HashMap<(i32, [i32; 2]), i32> = HashMap::new();
    dp.insert((0, [introverts_count, extroverts_count]), 0);
    let mut ret = 0;
    for _ in 0..m {
      let mut next_dp = HashMap::new();
      for (&(mask, counts), &score) in &dp {
        let mut sub_dp = HashMap::new();
        sub_dp.insert((0, counts), score);
        for j in 0..n {
          let mut insert_items: Vec<((i32, [i32; 2]), i32)> = vec![];
          for ((sub_mask, sub_counts), &sub_value) in &sub_dp {
            for k in 0..2 {
              let mut next_mask = *sub_mask;
              let mut next_counts = sub_counts.clone();
              let mut next_value = sub_value;
              if sub_counts[k] != 0 {
                next_mask |= (k as i32 + 1) << (2 * j);
                let top_left = [
                  (mask >> (2 * j)) & 3,
                  if j > 0 {
                    (sub_mask >> (2 * (j - 1))) & 3
                  } else {
                    0
                  },
                ];
                for neighbor in top_left {
                  match neighbor {
                    1 | 2 => {
                      if k == 0 {
                        next_value -= 30;
                      } else {
                        next_value += 20;
                      }

                      if neighbor == 1 {
                        next_value -= 30;
                      } else {
                        next_value += 20;
                      }
                    }
                    _ => {}
                  }
                }
                next_value += if k == 0 { 120 } else { 40 };
                if next_value > ret {
                  ret = next_value;
                }
                next_counts[k] -= 1;
              } else {
                continue;
              }
              if next_counts[0] + next_counts[1] != 0 {
                insert_items.push(((next_mask, next_counts), next_value));
              }
            }
          }
          for (key, item) in insert_items {
            sub_dp.insert(key, item);
          }
        }
        for (key, value) in sub_dp {
          if let Some(v) = next_dp.get_mut(&key) {
            if value > *v {
              *v = value;
            }
          } else {
            next_dp.insert(key, value);
          }
        }
      }
      dp = next_dp;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    m: i32,
    n: i32,
    introverts_count: i32,
    extroverts_count: i32,
    ret: i32,
  }

  #[test]
  fn test_get_max_grid_happiness_simple() {
    let suites = vec![
      Suite {
        m: 2,
        n: 3,
        introverts_count: 1,
        extroverts_count: 2,
        ret: 240,
      },
      Suite {
        m: 3,
        n: 1,
        introverts_count: 2,
        extroverts_count: 1,
        ret: 260,
      },
      Suite {
        m: 2,
        n: 2,
        introverts_count: 4,
        extroverts_count: 0,
        ret: 240,
      },
      Suite {
        m: 3,
        n: 4,
        introverts_count: 4,
        extroverts_count: 3,
        ret: 680,
      },
      Suite {
        m: 5,
        n: 5,
        introverts_count: 6,
        extroverts_count: 6,
        ret: 1240,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::get_max_grid_happiness(s.m, s.n, s.introverts_count, s.extroverts_count)
      );
    }
  }
}
