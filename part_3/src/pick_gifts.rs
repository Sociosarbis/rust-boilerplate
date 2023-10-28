use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<_> = gifts.into_iter().collect();
    for _ in 0..k {
      if let Some(num) = heap.pop() {
        heap.push((num as f64).sqrt() as i32);
      }
    }
    heap.into_iter().fold(0, |acc, num| acc + num as i64)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    gifts: Vec<i32>,
    k: i32,
    ret: i64,
  }

  #[test]
  fn test_pick_gifts_simple() {
    let suites = vec![
      Suite {
        gifts: vec![25, 64, 9, 4, 100],
        k: 4,
        ret: 29,
      },
      Suite {
        gifts: vec![1, 1, 1, 1],
        k: 4,
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::pick_gifts(s.gifts, s.k));
    }
  }
}
