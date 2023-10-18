use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<_> = nums.into_iter().collect();
    let mut ret = 0;
    for _ in 0..k {
      if let Some(num) = heap.pop() {
        ret += num as i64;
        heap.push((num / 3) + if num % 3 == 0 { 0 } else { 1 });
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
    k: i32,
    ret: i64,
  }

  #[test]
  fn test_max_kelements_simple() {
    let suites = vec![
      Suite {
        nums: vec![10, 10, 10, 10, 10],
        k: 5,
        ret: 50,
      },
      Suite {
        nums: vec![1, 10, 3, 3, 3],
        k: 3,
        ret: 17,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_kelements(s.nums, s.k));
    }
  }
}
