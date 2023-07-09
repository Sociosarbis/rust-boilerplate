use super::*;

use std::collections::{HashMap, HashSet};

impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
      if let Some(v) = map.get_mut(&num) {
        *v = (v.0, i);
      } else {
        map.insert(num, (i, i));
      }
    }
    let mut set = HashSet::new();
    for (i, &num) in nums.iter().enumerate() {
      if let Some(v) = map.get(&num) {
        if v.0 != i {
          continue;
        }
      }
      for j in i + 1..nums.len() - 1 {
        let target = -(num + nums[j]);
        if let Some(&(_, k)) = map.get(&target) {
          if k > j {
            let mut key = vec![num, nums[j], target];
            key.sort_unstable();
            if !set.contains(&key) {
              set.insert(key);
            }
          }
        }
      }
    }
    set.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_three_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![-1, 0, 1, 2, -1, -4],
        ret: t2_i![[-1, 0, 1], [-1, -1, 2]],
      },
      Suite {
        nums: vec![0, 1, 1],
        ret: t2_i![],
      },
      Suite {
        nums: vec![0, 0, 0],
        ret: t2_i![[0, 0, 0]],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::three_sum(s.nums));
    }
  }
}
