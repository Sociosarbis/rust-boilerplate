use crate::Solution;

impl Solution {
  pub fn remove_duplicates_80(nums: &mut Vec<i32>) -> i32 {
    let mut j = 1;
    let mut count = 1;
    for i in 1..nums.len() {
      if count == 2 {
        if nums[i] == nums[i - 1] {
          continue
        }
      }
      count = if nums[i] == nums[i - 1] { count + 1 } else  { 1 };
      nums[j] = nums[i];
      j +=1;
    }
    while nums.len() > j {
      nums.pop();
    }
    j as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
    ret_vec: Vec<i32>
  }

  #[test]
  fn remove_duplicates_80_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,1,1,2,2,3],
        ret: 5,
        ret_vec: vec![1,1,2,2,3]
      },
      Suite {
        nums: vec![0,0,1,1,1,1,2,3,3],
        ret: 7,
        ret_vec: vec![0,0,1,1,2,3,3]
      },
      Suite {
        nums: vec![1,1,1,2,2,2,3,3],
        ret: 6,
        ret_vec: vec![1,1,2,2,3,3]
      }
    ];
    
    for mut s in suites {
      assert_eq!(Solution::remove_duplicates_80(&mut s.nums), s.ret);
      assert_eq!(s.nums, s.ret_vec);
    }
  }
}