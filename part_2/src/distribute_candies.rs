use super::*;
use std::collections::HashSet;

impl Solution {
  pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let mut types = HashSet::new();
    for &t in &candy_type {
      types.insert(t);
    }
    (if types.len() > (candy_type.len() >> 1) { candy_type.len() >> 1 } else { types.len() }) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    candy_type: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_distribute_candies_simple() {
    let suites = vec![
      Suite {
        candy_type: vec![1,1,2,2,3,3],
        ret: 3
      },
      Suite {
        candy_type: vec![1,1,2,3],
        ret: 2
      },
      Suite {
        candy_type: vec![6,6,6,6],
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::distribute_candies(s.candy_type));
    }
  }

}