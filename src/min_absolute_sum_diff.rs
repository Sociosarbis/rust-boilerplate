use super::*;

impl Solution {
  pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mask = 1000000007;
    let mut nums1_copy = nums1.clone();
    let mut min_minus = 0;
    nums1_copy.sort();
    for i in 0..nums1.len() {
      let diff = (nums1[i] - nums2[i]).abs();
      if diff != 0 {
      let mut opt_diff: i32;
      let index = Solution::binary_search(&nums1_copy, nums2[i], true) as usize;
      if index < nums1_copy.len() {
        opt_diff = (nums1_copy[index] - nums2[i]).abs();
        if opt_diff - diff < min_minus {
          min_minus = opt_diff - diff;
        }
      }
      if index > 0 {
        opt_diff = (nums1_copy[index - 1] - nums2[i]).abs();
        if opt_diff - diff < min_minus {
          min_minus = opt_diff - diff;
        }
      }
      ret += diff;
        if ret >= mask {
            ret %= mask;
        }
      }
    }
    (ret + mask + min_minus) % mask
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_min_absolute_sum_diff_simple() {
    let suites = vec![
      Suite {
        nums1: vec![1,7,5],
        nums2: vec![2,3,5],
        ret: 3
      },
      Suite {
        nums1: vec![2,4,6,8,10],
        nums2: vec![2,4,6,8,10],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_absolute_sum_diff(s.nums1, s.nums2));
    }
  }
}