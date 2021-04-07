use super::Solution;

impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> bool {
    Solution::_search(&nums, target, 0, nums.len() as i32 - 1)
  }

  fn _search(nums: &Vec<i32>, target: i32, l: i32, r: i32) -> bool {
    if l > r {
      return false;
    }
    let left = l as usize;
    let right = r as usize;
    let mid = (left + right) / 2;
    if nums[mid] == target {
        return true;
    }
    let half_rotated = !(nums[left] <= nums[mid] && nums[mid] <= nums[right]);
    if half_rotated {
      if nums[left] <= nums[mid] && nums[left] <= target && target <= nums[mid] {
        return Solution::_binary_search(&nums, target, left as i32, mid as i32 - 1) != -1;
      } else if nums[mid] <= nums[right] && nums[mid] <= target && target <= nums[right] {
        return Solution::_binary_search(&nums, target, mid as i32 + 1, right as i32) != -1;
      }
    }
    if !half_rotated || nums[left] > nums[mid] {
      let ret = Solution::_search(nums, target, left as i32, mid as i32 - 1);
      if ret {
        return ret;
      }
    }
    if !half_rotated || nums[mid] > nums[right] {
      let ret = Solution::_search(nums, target, mid as i32 + 1, right as i32);
      if ret {
        return ret;
      }
    }
    false
  }

  fn _binary_search(nums: &Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
    if left > right {
      return -1;
    }
    let mut l = left as usize;
    let mut r = right as usize;
    while l <= r {
      let mid = (l + r) / 2;
      if nums[mid] < target {
        l = mid + 1;
        if l > r {
          return -1
        }
      } else if nums[mid] > target {
        if mid == 0 || mid - 1 < l {
          return -1;
        }
        r = mid - 1;
      } else {
        return mid as i32;
      }
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
    ret: bool
  }

  #[test]
  fn test_search_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,5,6,0,0,1,2],
        target: 0,
        ret: true
      },
      Suite {
        nums: vec![2,5,6,0,0,1,2],
        target: 3,
        ret: false
      },
      Suite {
        nums: vec![1,0,1,1,1],
        target: 0,
        ret: true
      },
      Suite {
        nums: vec![1],
        target: 0,
        ret: false
      },
      Suite {
        nums: vec![1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1],
        target: 2,
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(Solution::search(s.nums, s.target), s.ret);
    }
  }
}