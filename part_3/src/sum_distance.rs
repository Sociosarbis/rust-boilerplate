use super::*;

impl Solution {
  pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
    let mask = 1e9 as i32 + 7;
    let mut nums: Vec<i64> = nums.into_iter().map(|num| num as i64).collect();
    for (i, c) in s.chars().enumerate() {
      if c == 'L' {
        nums[i] -= d as i64;
      } else {
        nums[i] += d as i64;
      }
    }
    nums.sort_unstable();
    let nums: Vec<i32> = nums
      .into_iter()
      .map(|num| (num % mask as i64) as i32)
      .collect();
    let sum = nums.iter().fold(0, |acc, num| (acc + num) % mask);
    let mut ret = sum;
    let mut acc = 0;
    for num in nums.into_iter().rev() {
      ret = ((ret + acc) % mask - sum) % mask;
      acc = (acc + (2 * num) % mask) % mask;
    }
    if ret < 0 {
      return ret + mask;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    s: String,
    d: i32,
    ret: i32,
  }

  #[test]
  fn test_sum_distance_simple() {
    let suites = vec![
      Suite {
        nums: vec![-2, 0, 2],
        s: "RLL".to_string(),
        d: 3,
        ret: 8,
      },
      Suite {
        nums: vec![1, 0],
        s: "RL".to_string(),
        d: 2,
        ret: 5,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::sum_distance(s.nums, s.s, s.d));
    }
  }
}
