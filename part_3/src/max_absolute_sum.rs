use super::*;

impl Solution {
  pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut temp = 0;
    for &num in &nums {
      if temp + num <= 0 {
        temp = 0;
      } else {
        temp += num;
        if temp > ret {
          ret = temp;
        }
      }
    }
    temp = 0;
    for &num in &nums {
      if temp + num >= 0 {
        temp = 0;
      } else {
        temp += num;
        if -temp > ret {
          ret = -temp;
        }
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
  fn test_max_absolute_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, -3, 2, 3, -4],
        ret: 5,
      },
      Suite {
        nums: vec![2, -5, 1, -4, 3, -2],
        ret: 8,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_absolute_sum(s.nums));
    }
  }
}
