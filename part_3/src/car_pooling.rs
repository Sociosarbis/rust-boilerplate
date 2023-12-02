use super::*;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl Solution {
  pub fn car_pooling(mut trips: Vec<Vec<i32>>, mut capacity: i32) -> bool {
    trips.sort_unstable_by(|a, b| match a[1].cmp(&b[1]) {
      Ordering::Equal => a[2].cmp(&b[2]),
      res => res,
    });
    let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    for trip in trips {
      while let Some((Reverse(to), num)) = heap.pop() {
        if to <= trip[1] {
          capacity += num;
        } else {
          heap.push((Reverse(to), num));
          break;
        }
      }
      if trip[0] > capacity {
        return false;
      }
      capacity -= trip[0];
      heap.push((Reverse(trip[2]), trip[0]));
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    trips: Vec<Vec<i32>>,
    capacity: i32,
    ret: bool,
  }

  #[test]
  fn test_car_pooling_simple() {
    let suites = vec![
      Suite {
        trips: t2_i![[2, 1, 5], [3, 3, 7]],
        capacity: 4,
        ret: false,
      },
      Suite {
        trips: t2_i![[2, 1, 5], [3, 3, 7]],
        capacity: 5,
        ret: true,
      },
      Suite {
        trips: t2_i![[3,2,7],[3,7,9],[8,3,9]],
        capacity: 11,
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::car_pooling(s.trips, s.capacity));
    }
  }
}
