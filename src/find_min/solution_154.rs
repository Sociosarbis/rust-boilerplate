use crate::Solution;


impl Solution {
  pub fn find_min_154(nums: Vec<i32>) -> i32 {
    Solution::_find_min_154(&nums, 0, nums.len() as i32 - 1, 5000)
  }

  fn _find_min_154(nums: &Vec<i32>, left: i32, right: i32, min: i32) -> i32 {
    let l = left as usize;
    let r = right as usize;
    let mid = (l + r) / 2;
    let mut ret = min;
    if nums[mid] < ret {
      ret = nums[mid]
    }
    
    if mid > 0 && mid - 1 >= l {
      let ret_1 = if nums[l] < nums[mid - 1] { nums[l] } else { Solution::_find_min_154(nums, left, mid as i32 - 1, ret) };
      if ret_1 < ret {
        ret = ret_1;
      }
    }

    if mid + 1 <= r {
      let ret_2 = if nums[mid + 1] < nums[r] { nums[mid + 1] } else { Solution::_find_min_154(nums, mid as i32 + 1, right, ret) };
      if ret_2 < ret {
        ret = ret_2;
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
    ret: i32
  }

  #[test]
  fn test_find_min_154_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,3,5],
        ret: 1,
      },
      Suite {
        nums: vec![2,2,2,0,1],
        ret: 0
      },
    ];

    for s in suites {
      assert_eq!(Solution::find_min_154(s.nums), s.ret);
    }
  }
}
