use super::*;

impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut temp = 0;
    let mut ret = nums[0];
    for num in nums {
      temp = if temp > 0 { temp + num } else { num };
      if temp > ret {
        ret = temp;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_max_sub_array_simple() {
    let suites = vec![
      Suite {
        nums: vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        ret: 6,
      },
      Suite {
        nums: vec![1],
        ret: 1,
      },
      Suite {
        nums: vec![5, 4, -1, 7, 8],
        ret: 23,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sub_array(s.nums));
    }
  }
}
