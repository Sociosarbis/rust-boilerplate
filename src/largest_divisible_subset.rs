use super::*;
use std::cmp::max;


impl Solution {
  pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    let mut dp: Vec<Vec<i32>> = nums.iter().map(|&num| { vec![num]} ).collect();
    let mut ret = 0;
    for i in 1..nums.len() {
      let mut k = i;
      let num = nums[i];
      let divisor_max = num / nums[0] + 1;
      let mut r = i - 1;
      let mut divisor = 2;
      while divisor < divisor_max {
        let target = num / divisor;
        let j = Solution::binary_search_general(&nums, target, 0, r, true) as usize;
        if num % divisor == 0 {
          if j <= r && nums[j] == target && (dp[j].len() > dp[k].len() || k == i) {
            k = j;
          }
        }
        if j > 0 {
          r = j - 1;
        }
        divisor = max(divisor + 1, num / nums[r]);
      }
      if k != i {
        dp[i] = dp[k].clone();
        dp[i].push(num);
        if dp[i].len() > dp[ret].len() {
          ret = i;
        }
      }
    }
    dp[ret].clone()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_largest_divisible_subset_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3],
        ret: vec![1,2]
      },
      Suite {
        nums: vec![1,2,4,8],
        ret: vec![1,2,4,8]
      },
      Suite {
        nums: vec![2,3,5,7,11,13,17,19,23,31,1000000007],
        ret: vec![2]
      }
    ];

    for s in suites {
      assert_eq!(Solution::largest_divisible_subset(s.nums), s.ret);
    }
  }
}