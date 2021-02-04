use super::Solution;

impl Solution {
  pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut ret = nums[0..k as usize].iter().fold(0.0, |acc, &item| { acc + item as f64 });
    let mut tmp = ret;
    for i in k as usize..nums.len() {
      tmp += nums[i] as f64;
      tmp -= nums[i - k as usize] as f64;
      if tmp > ret {
        ret = tmp;
      }
    }
    ret / k as f64
  }
}


#[cfg(tests)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: f64
  }

  #[test]
  fn find_max_average_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,12,-5,-6,50,3],
        k: 4,
        ret: 12.75
      }
    ];

    for s in suites {
      assert_eq!(Solution::find_max_average(s.nums, s.k), s.ret);
    }
  }

}