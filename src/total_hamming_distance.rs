use super::Solution;

impl Solution {
  pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 0..30 {
      let base = 1 << i;
      let mut ones = 0;
      for num in &nums {
        if (num & base) != 0 {
          ones += 1;
        }
      }
      ret += ones * (nums.len() - ones)
    }
    ret as i32
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
  fn test_total_hamming_distance_simple() {
    let suites = vec![
      Suite {
        nums: vec![4, 14, 2],
        ret: 6
      },
      Suite {
        nums: vec![4, 14, 4],
        ret: 4
      }
    ];

    for s in suites {
      assert_eq!(Solution::total_hamming_distance(s.nums), s.ret);
    }
  }
}