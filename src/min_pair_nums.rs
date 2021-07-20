use super::Solution;

impl Solution {
  pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    nums.sort();
    for i in 0..nums.len() / 2 {
      let temp = nums[i] + nums[nums.len() - 1 - i];
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
    ret: i32
  }

  #[test]
  fn test_min_pair_num_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,5,2,3],
        ret: 7
      },
      Suite {
        nums: vec![3,5,4,2,4,6],
        ret: 8
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_pair_sum(s.nums));
    }
  }
}