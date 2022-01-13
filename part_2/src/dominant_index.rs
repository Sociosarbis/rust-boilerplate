use super::*;

impl Solution {
  pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut min_max = 0;
    for j in 1..nums.len() {
      if nums[j] > nums[i] {
        min_max = nums[i] << 1;
        i = j;
      } else if (nums[j] << 1) > min_max {
        min_max = nums[j] << 1;
      }
    }
    if nums[i] >= min_max { i as i32 } else { -1 }
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
  fn test_dominant_index_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,6,1,0],
        ret: 1
      },
      Suite {
        nums: vec![1,2,3,4],
        ret: -1
      },
      Suite {
        nums: vec![1],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::dominant_index(s.nums));
    }
  }
}