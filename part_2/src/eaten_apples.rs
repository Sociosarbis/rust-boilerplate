use super::*;

use std::cmp::Reverse;
use std::collections::{HashMap, BinaryHeap};

impl Solution {
  pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut queue = BinaryHeap::new();
    let mut day_to_count = HashMap::new();
    let mut i = 0;
    loop {
      if i < apples.len() && apples[i] != 0 {
        let day = i as i32 + days[i];
        if let Some(count) = day_to_count.get_mut(&day) {
          *count += apples[i];
        } else {
          day_to_count.insert(day, apples[i]);
          queue.push(Reverse(day));
        }
      }
      while let Some(item) = queue.peek() {
        let day = item.0;
        if day <= i as i32 {
          queue.pop();
        } else {
          if let Some(count) = day_to_count.get_mut(&day) {
            *count -= 1;
            ret += 1;
            if *count == 0 {
              day_to_count.remove(&day);
              queue.pop();
            }
          }
          break;
        }
      }
      i += 1;
      if i >= apples.len() && queue.is_empty() {
        break;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;


  struct Suite {
    apples: Vec<i32>,
    days: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_eaten_apples_simple() {
    let suites = vec![
      Suite {
        apples: vec![1,2,3,5,2],
        days: vec![3,2,1,4,2],
        ret: 7
      },
      Suite {
        apples: vec![3,0,0,0,0,2],
        days: vec![3,0,0,0,0,2],
        ret: 5
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::eaten_apples(s.apples, s.days));
    }
  }
}