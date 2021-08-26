use super::*;

impl Solution {
  pub fn search_lcof(nums: Vec<i32>, target: i32) -> i32 {
    let i = Solution::binary_search(&nums, target, false);
    if i != -1 {
      let mut ret = 1;
      let mut left = i as usize;
      let mut right = i as usize;
      while left > 0 && nums[left - 1] == target {
        ret += 1;
        left -= 1;
      }

      while right + 1 < nums.len() && nums[right + 1] == target {
        ret += 1;
        right += 1;
      }
      return ret;
    }
    0
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    target: i32,
    ret: i32
  }

  #[test]
  fn test_search_lcof_simple() {
    let suites = vec![
      Suite {
        nums: vec![5,7,7,8,8,10],
        target: 8,
        ret: 2
      },
      Suite {
        nums: vec![5,7,7,8,8,10],
        target: 6,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::search_lcof(s.nums, s.target));
    }
  }
}