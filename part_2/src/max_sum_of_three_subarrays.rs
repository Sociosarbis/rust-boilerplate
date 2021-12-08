use super::*;

impl Solution {
  pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {

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
  fn test_max_sum_of_three_subarrays_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,1,2,6,7,5,1],
        k: 2,
        ret: vec![0,3,5]
      },
      Suite {
        nums: vec![1,2,1,2,1,2,1,2,1],
        k: 2,
        ret: vec![0,2,4]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sum_of_three_subarrays(s.nums, s.k));
    }
  }
}