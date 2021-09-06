use super::*;


impl Solution {
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    Solution::binary_search(&nums, target, false)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    target: i32,
    ret: i32
  }

  #[test]
  fn test_search_simple() {
    let suites = vec![
      Suite {
        nums: vec![-1,0,3,5,9,12],
        target: 9,
        ret: 4
      },
      Suite {
        nums: vec![-1,0,3,5,9,12],
        target: 2,
        ret: -1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::search(s.nums, s.target));
    }
  }
}