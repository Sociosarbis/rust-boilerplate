use super::*;

impl Solution {
  pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    (k + 2 * nums.into_iter().max().unwrap() - 1) * k / 2
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
  fn test_maximize_sum_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 2, 3, 4, 5],
        k: 3,
        ret: 18,
      },
      Suite {
        nums: vec![5, 5, 5],
        k: 2,
        ret: 11,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximize_sum(s.nums, s.k));
    }
  }
}
