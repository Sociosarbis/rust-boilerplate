use super::Solution;

impl Solution {
  pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut max = nums[0];
    let mut l = -1;
    let mut r = -1;
    for i in 1..nums.len() {
      if nums[i] < max {
        if l == -1 || nums[i] < l {
          l = nums[i];
        }

        if r == -1 || nums[i] > r {
          r = nums[i];
        }
      }
      
      if nums[i] > max {
        max = nums[i];
      } else if nums[i] < min {
        min = nums[i];
      }
    }

    if l != -1 {
      for i in 0..nums.len() {
        if nums[i] > l {
          let mut end = i;
          for j in i + 1..nums.len() {
            if nums[j] <= r {
              end = j;
            }
          }
          return (end - i + 1) as i32;
        }
      }
    }
    0
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
  fn test_find_unsorted_subarray_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,6,4,8,10,9,15],
        ret: 5
      },
      Suite {
        nums: vec![1,2,3,4],
        ret: 0
      },
      Suite {
        nums: vec![1],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_unsorted_subarray(s.nums));
    }
  }
}