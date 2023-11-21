use super::*;

impl Solution {
  pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut i = 0;
    while i + 1 < nums.len() {
      if nums[i] != nums[i + 1] {
        count += 2;
        i += 2;
        continue;
      }
      i += 1;
    }
    nums.len() as i32 - count
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
  fn test_min_deletion_simple() {
    let suites = vec![
      Suite {
        nums: vec![1, 1, 2, 3, 5],
        ret: 1,
      },
      Suite {
        nums: vec![1, 1, 2, 2, 3, 3],
        ret: 2,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_deletion(s.nums));
    }
  }
}
