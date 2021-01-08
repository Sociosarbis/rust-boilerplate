use super::Solution;

impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if k != 0 {
      let n = nums.len();
      let mut next_i = 0;
      let mut target_i = k as usize % n;
      let mut count = n as i32;
      let mut next_target_i: usize;
      while count > 0  {
        next_target_i = (target_i + k as usize) % n;
        let tmp = nums[next_i];
        nums[next_i] = nums[target_i];
        nums[target_i] = tmp;
        if next_i == next_target_i {
          count -= 1;
          next_i += 1;
          next_target_i = (next_i + k as usize) % n;
        }
        target_i = next_target_i;
        count -= 1;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: Vec<i32>
  }

  #[test]
  fn rotate_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3,4,5,6,7],
        k: 3,
        ret: vec![5,6,7,1,2,3,4]
      },
      Suite {
        nums: vec![-1,-100,3,99],
        k: 2,
        ret: vec![3,99,-1,-100]
      }
    ];
    for s in suites {
      let mut nums = s.nums;
      Solution::rotate(&mut nums, s.k);
      assert_eq!(nums, s.ret)
    }
  }
}