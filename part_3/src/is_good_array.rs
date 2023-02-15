use super::*;


impl Solution {
  pub fn is_good_array(nums: Vec<i32>) -> bool {
    let mut d = nums[0];
    for i in 1..nums.len() {
      d = Solution::get_greatest_common_divisor(d, nums[i]);
      if d == 1 {
        break;
      }
    }
    d == 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: bool
  }

  #[test]
  fn test_is_good_array_simple() {
    let suites = vec![
      Suite {
        nums: vec![12,5,7,23],
        ret: true
      },
      Suite {
        nums: vec![29,6,10],
        ret: true
      },
      Suite {
        nums: vec![3,6],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_good_array(s.nums));
    }
  }
}