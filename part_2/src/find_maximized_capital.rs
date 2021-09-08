use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut list = vec![];
    for i in 0..profits.len() {
      list.push((capital[i], profits[i]));
    }
    list.sort_unstable_by(|a, b| {
        a.0.cmp(&b.0)
    });
    let mut i = 0;
    let mut queue = BinaryHeap::new();
    let mut ret = w;
    for _ in 0..k {
      while i < list.len() && list[i].0 <= ret {
        queue.push(list[i].1);
        i += 1;
      }
      if !queue.is_empty() {
        ret += queue.pop().unwrap();
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    k: i32,
    w: i32,
    profits: Vec<i32>,
    capital: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_find_maximized_capital_simple() {
    let suites = vec![
      Suite {
        k: 2,
        w: 0,
        profits: vec![1,2,3],
        capital: vec![0,1,1],
        ret: 4
      },
      Suite {
        k: 3,
        w: 0,
        profits: vec![1,2,3],
        capital: vec![0,1,2],
        ret: 6
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_maximized_capital(s.k, s.w, s.profits, s.capital));
    }
  }
}