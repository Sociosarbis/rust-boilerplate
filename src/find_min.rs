use super::Solution;


impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
    Solution::_find_min(&nums, 0, nums.len() as i32 - 1, 5000)
  }

  fn _find_min(nums: &Vec<i32>, left: i32, right: i32, min: i32) -> i32 {
    let l = left as usize;
    let r = right as usize;
    let mid = (l + r) / 2;
    let mut ret = min;
    if nums[mid] < ret {
      ret = nums[mid]
    }
    
    if mid > 0 && mid - 1 >= l {
      let ret_1 = if nums[l] <= nums[mid - 1] { nums[l] } else { Solution::_find_min(nums, left, mid as i32 - 1, ret) };
      if ret_1 < ret {
        ret = ret_1;
      }
    }

    if mid + 1 <= r {
      let ret_2 = if nums[mid + 1] <= nums[r] { nums[mid + 1] } else { Solution::_find_min(nums, mid as i32 + 1, right, ret) };
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
  fn test_search_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,4,5,1,2],
        ret: 1,
      },
      Suite {
        nums: vec![4,5,6,7,0,1,2],
        ret: 0
      },
      Suite {
        nums: vec![11,13,15,17],
        ret: 11
      }
    ];

    for s in suites {
      assert_eq!(Solution::find_min(s.nums), s.ret);
    }
  }
}
