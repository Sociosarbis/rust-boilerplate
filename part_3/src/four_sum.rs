use super::*;

impl Solution {
  pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    Solution::n_sum(&nums, 0, 4, target as i64)
  }

  fn n_sum(nums: &Vec<i32>, l: usize, n: i32, target: i64) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    for i in l..nums.len() {
      let num = nums[i];
      if (num as i64 * n as i64) > target {
        break;
      }
      if i != l && nums[i - 1] == num {
        continue;
      }
      let temp = target - num as i64;
      let res = if n == 3 {
        Solution::n_two_sum(nums, i + 1, temp)
      } else {
        Solution::n_sum(nums, i + 1, n - 1, temp)
      };
      if !res.is_empty() {
        ret.append(
          &mut res
            .into_iter()
            .map(|mut item| {
              item.insert(0, num);
              item
            })
            .collect(),
        )
      }
    }
    ret
  }

  fn n_two_sum(nums: &Vec<i32>, mut l: usize, target: i64) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut r = nums.len() - 1;
    while l < r {
      let temp = (nums[l] + nums[r]) as i64;
      if temp > target {
        r -= 1;
      } else if temp < target {
        l += 1;
      } else {
        ret.push(vec![nums[l], nums[r]]);
        l += 1;
        while l < r && nums[l] == nums[l - 1] {
          l += 1;
        }
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
    target: i32,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_four_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 0, -1, 0, -2, 2],
        target: 0,
        ret: t2_i![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]],
      },
      Suite {
        nums: vec![2, 2, 2, 2, 2],
        target: 8,
        ret: t2_i![[2, 2, 2, 2]],
      },
      Suite {
        nums: vec![0, 0, 0, -1000000000, -1000000000, -1000000000, -1000000000],
        target: -1000000000,
        ret: t2_i![[-1000000000, 0, 0, 0]],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::four_sum(s.nums, s.target));
    }
  }
}
