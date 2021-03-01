struct NumArray {
  sum_left: Vec<i32>,
  sum_right: Vec<i32>
}

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
      if nums.is_empty() {
        return NumArray {
          sum_left: vec![],
          sum_right: vec![],
        }
      }
      let mut sum_left = vec![0;nums.len()];
      let mut sum_right = vec![0;nums.len()];
      sum_left[0] = nums[0];
      for i in 1..nums.len() {
        sum_left[i] += sum_left[i - 1] + nums[i];
      }
      sum_right[nums.len() - 1] = nums[nums.len() - 1];
      for i in (0..nums.len() - 1).rev() {
        sum_right[i] += sum_right[i + 1] + nums[i];
      }
      
      NumArray {
        sum_left: sum_left,
        sum_right: sum_right,
      }
    }
    
    fn sum_range(&self, i: i32, j: i32) -> i32 {
      if self.sum_left.is_empty() {
        return 0;
      }
      return self.sum_left[self.sum_left.len() - 1] -
        (if i > 0 { self.sum_left[i as usize - 1] } else { 0 })
        - (if j < self.sum_right.len() as i32 - 1 { self.sum_right[j as usize + 1] } else { 0 })
      ;
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Solution;

  struct Suite {
    nums: Vec<i32>,
    ranges: Vec<Vec<i32>>,
    rets: Vec<i32>
  }

  #[test]
  fn sum_range_simple() {
    let suites = vec![
      Suite {
        nums: vec![-2, 0, 3, -5, 2, -1],
        ranges: Solution::t2_i(vec![&[0, 2], &[2, 5], &[0, 5]]),
        rets: vec![1, -1, -3]
      }
    ];

    for s in suites {
      let num_array = NumArray::new(s.nums);
      for i in 0..s.ranges.len() {
        assert_eq!(num_array.sum_range(s.ranges[i][0], s.ranges[i][1]), s.rets[i]);
      }
    }
  }
}