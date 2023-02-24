use super::*;

impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut m = [false;101];
    let mut ret = 0;
    for &num in &nums {
      if num != 0 {
        if !m[num as usize] {
          m[num as usize] = true;
          ret += 1;
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
    ret: i32
  }

  #[test]
  fn test_minimum_operations_simple() {
    let suites = vec![
      Suite {
        nums: vec![1,5,0,3,5],
        ret: 3
      },
      Suite {
        nums: vec![0],
        ret: 0
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::minimum_operations(s.nums));
    }
  }
}