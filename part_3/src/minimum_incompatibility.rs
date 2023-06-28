use std::collections::HashMap;

use super::*;

impl Solution {
  pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    if k == nums.len() as i32 {
      return 0;
    }
    let mut dp: HashMap<u64, i32> = HashMap::new();
    let mut counter = [0; 17];
    for &num in &nums {
      let index = num as usize;
      counter[index] += 1;
      if counter[index] > k {
        return -1;
      }
    }
    Solution::minimum_incompatibility_dfs(&mut counter, &mut dp, 0, 0, 0, 0, k, nums.len() as i32)
  }

  fn minimum_incompatibility_dfs(
    counter: &mut [i32; 17],
    dp: &mut HashMap<u64, i32>,
    mask: i32,
    min: i32,
    max: i32,
    i: i32,
    k: i32,
    size: i32,
  ) -> i32 {
    let mut ret = 0;
    if i < size {
      let group_size = size / k;
      let index = i % group_size;
      let mut h: u64 = 0;
      if index == 0 {
        h = {
          let mut out = 0;
          for i in 1..=16 {
            if counter[i as usize] != 0 {
              out |= (counter[i as usize] as u64) << ((i - 1) * 4);
            }
          }
          out
        };
        if let Some(&v) = dp.get(&h) {
          return v;
        }
      }
      let mut local_min = -1;
      for num in 1..=16 {
        if counter[num as usize] != 0 && mask & (1 << (num - 1)) == 0 {
          counter[num as usize] -= 1;
          let res = if index == 0 {
            Solution::minimum_incompatibility_dfs(
              counter,
              dp,
              mask | (1 << (num - 1)),
              num,
              num,
              i + 1,
              k,
              size,
            )
          } else if index + 1 == group_size {
            let temp = max.max(num) - min.min(num);
            let r = Solution::minimum_incompatibility_dfs(counter, dp, 0, 0, 0, i + 1, k, size);
            if r == -1 {
              -1
            } else {
              temp + r
            }
          } else {
            Solution::minimum_incompatibility_dfs(
              counter,
              dp,
              mask | (1 << (num - 1)),
              min.min(num),
              max.max(num),
              i + 1,
              k,
              size,
            )
          };
          counter[num as usize] += 1;
          if res != -1 {
            if local_min == -1 || local_min > res {
              local_min = res;
            }
          }
        }
      }
      ret = local_min;
      if index == 0 {
        dp.insert(h, ret);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_minimum_incompatibility_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 2, 1, 4],
        k: 2,
        ret: 4,
      },
      Suite {
        nums: vec![6, 3, 8, 1, 3, 1, 2, 2],
        k: 4,
        ret: 6,
      },
      Suite {
        nums: vec![5, 3, 3, 6, 3, 3],
        k: 3,
        ret: -1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::minimum_incompatibility(s.nums, s.k));
    }
  }
}
