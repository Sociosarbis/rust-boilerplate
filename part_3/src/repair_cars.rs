use super::*;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
  pub fn repair_cars(ranks: Vec<i32>, mut cars: i32) -> i64 {
    let mut queue: BinaryHeap<Reverse<(i64, i32, i32)>> = BinaryHeap::new();
    for rank in ranks {
      queue.push(Reverse((rank as i64, rank, 1)));
    }
    while let Some(Reverse((t, r, c))) = queue.pop() {
      cars -= 1;
      if cars == 0 {
        return t;
      }
      queue.push(Reverse((r as i64 * ((c + 1) as i64).pow(2), r, c + 1)));
    }
    unreachable!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    ranks: Vec<i32>,
    cars: i32,
    ret: i64,
  }

  #[test]
  fn test_repair_cars_simple() {
    let suites = vec![
      Suite {
        ranks: vec![4, 2, 3, 1],
        cars: 10,
        ret: 16,
      },
      Suite {
        ranks: vec![5, 1, 8],
        cars: 6,
        ret: 16,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::repair_cars(s.ranks, s.cars));
    }
  }
}
