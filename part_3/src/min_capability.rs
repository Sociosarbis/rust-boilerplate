use super::*;

impl Solution {
  pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    let mut l = 1;
    let mut r = 1e9 as i32;
    'a: while l <= r {
      let mid = (l + r) >> 1;
      let mut dp = [0, if nums[0] > mid { 0 } else { 1 }];
      if dp[1] >= k {
        r = mid - 1;
        continue 'a;
      }
      for &num in nums.iter().skip(1) {
        dp = [dp[0].max(dp[1]), dp[0] + if num > mid { 0 } else { 1 }];
        if dp[0].max(dp[1]) >= k {
          r = mid - 1;
          continue 'a;
        }
      }
      l = mid + 1;
    }
    l
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_min_capability_simple() {
    let suites = vec![
       Suite {
        nums: vec![2, 3, 5, 9],
        k: 2,
        ret: 5,
      },
      Suite {
        nums: vec![2, 7, 9, 3, 1],
        k: 2,
        ret: 2,
      },
      Suite {
        nums: vec![1],
        k: 1,
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_capability(s.nums, s.k));
    }
  }
}
