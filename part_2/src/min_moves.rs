use super::*;

impl Solution {
  pub fn min_moves(nums: Vec<i32>) -> i32 {
    (nums.iter().map(|&item| item as i64).sum::<i64>()
      - *nums.iter().min().unwrap() as i64 * nums.len() as i64) as i32
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
  fn test_min_moves_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,2,3],
        ret: 3
      },
      Suite {
        nums: vec![1,1,1],
        ret: 0
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_moves(s.nums));
    }
  }
}