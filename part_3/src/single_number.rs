use super::*;

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let first = nums[0];
    nums.into_iter().skip(1).fold(first, |acc, num| acc ^ num)
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
  fn test_single_number_simple() {
    let suites = vec![
      Suite {
        nums: vec![2, 2, 1],
        ret: 1,
      },
      Suite {
        nums: vec![4, 1, 2, 1, 2],
        ret: 4,
      },
      Suite {
        nums: vec![1],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::single_number(s.nums));
    }
  }
}
