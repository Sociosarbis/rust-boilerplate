use super::*;

impl Solution {
  pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i <= j {
      let mid  = (i + j) / 2;
      let bg_left = if mid == 0 { true } else { nums[mid] > nums[mid - 1] };
      let bg_right = if mid + 1 == nums.len() { true } else { nums[mid] > nums[mid + 1] };
      if bg_left {
        if bg_right {
          i = mid;
          break;
        } else {
          i = mid + 1;
        }
      } else {
        if bg_right {
          j = mid - 1;
        } else {
          i = mid + 1;
        }
      }
    }
    i as i32
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
  fn test_find_peak_element_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,1],
        ret: 2
      },
      Suite {
        nums: vec![1,2,1,3,5,6,4],
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_peak_element(s.nums));
    }
  }
}
