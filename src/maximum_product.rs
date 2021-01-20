use super::Solution;

impl Solution {
  pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 3 { return nums[0] * nums[1] * nums[2]; }
    nums.sort_unstable();
    let left = nums[n - 3] * nums[n - 2] * nums[n - 1];
    let right = nums[0] * nums[1] * nums[n - 1];
    return if left < right { right } else { left }
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
  fn maximum_product_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3],
        ret: 6
      },
      Suite {
        nums: vec![1,2,3,4],
        ret: 24
      },
      Suite {
        nums: vec![-1,-2,-3],
        ret: -6
      }
    ];
    for s in suites {
      assert_eq!(Solution::maximum_product(s.nums), s.ret);
    }
  }
}