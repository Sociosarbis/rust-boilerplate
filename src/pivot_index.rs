use super::Solution;

impl Solution {
  pub fn pivot_index(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return -1;
    }
    let mut left = 0;
    let mut right = nums.iter().fold(0, |acc, num| { acc + num });
    right -= nums[0];
    if left == right {
      return 0;
    }
    for i in 1..nums.len() {
      left += nums[i - 1];
      right -= nums[i];
      if left == right {
        return i as i32;
      }
    }
    -1
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
  fn pivot_index_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,7,3,6,5,6],
        ret: 3
      },
      Suite {
        nums: vec![1,2,3],
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::pivot_index(s.nums), s.ret);
    }
  }
}