use super::*;

impl Solution {
  pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut dp = [0; 3];
    for num in nums {
      let temp = dp.map(|item| item + num);
      for item in temp {
        if item > dp[(item % 3) as usize] {
          dp[(item % 3) as usize] = item;
        }
      }
    }
    dp[0]
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
  fn test_max_sum_div_three_simple() {
    let suites = vec![
      Suite {
        nums: vec![3, 6, 5, 1, 8],
        ret: 18,
      },
      Suite {
        nums: vec![4],
        ret: 0,
      },
      Suite {
        nums: vec![1, 2, 3, 4, 4],
        ret: 12,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sum_div_three(s.nums));
    }
  }
}
