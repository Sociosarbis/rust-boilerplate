use super::Solution;

impl Solution {
  pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut ret = 0;

    let mut j = 0;
    let mut temp = nums[0];
    for i in 0..nums.len() {
      if j < i {
        temp = nums[i];
        j = i;
      }
      while temp < goal && j + 1 < nums.len() {
        j += 1;
        temp += nums[j];
      }
      if temp == goal {
        ret += 1;
        let old_j = j;
        while j + 1 < nums.len() && nums[j + 1] == 0 {
          ret += 1;
          j += 1;
        }
        j = old_j;
      }
      temp -= nums[i];
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    goal: i32,
    ret: i32
  }

  #[test]
  fn test_num_subarrays_with_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,0,1,0,1],
        goal: 2,
        ret: 4
      },
      Suite {
        nums: vec![0,0,0,0,0],
        goal: 0,
        ret: 15
      },
      Suite {
        nums: vec![0,0,0,0,1],
        goal: 2,
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_subarrays_with_sum(s.nums, s.goal));
    }
  }
}