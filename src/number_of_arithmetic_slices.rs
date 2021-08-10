use super::Solution;

impl Solution {
  pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
      return 0;
    }
    let mut ret = 0;
    let mut delta = nums[1] - nums[0];
    let mut i = 0;
    for j in 2..nums.len() {
      if nums[j] - nums[j - 1] != delta {
        if j - i > 2 {
          ret += (j - i - 2) * (j - i - 1) / 2 
        }
        i = j - 1;
        delta = nums[j] - nums[i];
      }
    }
    if nums.len() - i > 2 {
      ret += (nums.len() - i - 2) * (nums.len() - i - 1) / 2 
    }
    ret as i32
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
  fn test_number_of_arithmetic_slices_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,4],
        ret: 3
      },
      Suite {
        nums: vec![1],
        ret: 0
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::number_of_arithmetic_slices(s.nums));
    }
  }
}