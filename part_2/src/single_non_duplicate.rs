use super::*;

impl Solution {
  pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
      let mid = (l + r) >> 1;
      if mid > 0 && nums[mid - 1] == nums[mid] {
        if (mid - 1 - l) & 1 != 0 {
          r = mid - 2;
        } else {
          l = mid + 1;
        }
      } else if mid + 1 < nums.len() && nums[mid + 1] == nums[mid] {
        if (r - mid - 1) & 1 != 0 {
          l = mid + 2;
        } else {
          r = mid - 1;
        }
      } else {
        return nums[mid];
      }
    }
    nums[l]
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
  fn test_single_non_duplicate_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,1,2,3,3,4,4,8,8],
        ret: 2
      },
      Suite {
        nums: vec![3,3,7,7,10,11,11],
        ret: 10
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::single_non_duplicate(s.nums));
    }
  }
}