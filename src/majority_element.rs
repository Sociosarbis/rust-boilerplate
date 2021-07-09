use super::Solution;

impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    let mut count = 0;
    for &num in &nums {
      if ret == -1 {
        ret = num;
        count = 1;
      } else if ret == num {
        count += 1;
      } else {
        count -= 1;
        if count == 0 {
          ret = -1;
        }
      }
    }
    if ret != -1 {
      count = 0;
      for &num in &nums {
        if num == ret {
          count += 1;
        }
      }
      if count > nums.len() >> 1 {
        return ret;
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
  fn test_majority_element_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,5,9,5,9,5,5,5],
        ret: 5
      },
      Suite {
        nums: vec![3,2],
        ret: -1
      },
      Suite {
        nums: vec![2,2,1,1,1,2,2],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::majority_element(s.nums));
    }
  }
}