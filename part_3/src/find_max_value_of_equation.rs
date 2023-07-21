use super::*;

use std::collections::BinaryHeap;

impl Solution {
  pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut queue: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut ret = -1e9 as i32;
    for p in points {
      while let Some(v) = queue.pop() {
        if p[0] - v.1 <= k {
          queue.push(v);
          break;
        }
      }
      if let Some(v) = queue.peek() {
        if v.0 + p[0] + p[1] > ret {
          ret = v.0 + p[0] + p[1];
        }
      }
      queue.push((p[1] - p[0], p[0]));
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    points: Vec<Vec<i32>>,
    k: i32,
    ret: i32,
  }

  #[test]
  fn test_find_max_value_of_equation_simple() {
    let suites = vec![
      Suite {
        points: t2_i![[1, 3], [2, 0], [5, 10], [6, -10]],
        k: 1,
        ret: 4,
      },
      Suite {
        points: t2_i![[0, 0], [3, 0], [9, 2]],
        k: 3,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_max_value_of_equation(s.points, s.k));
    }
  }
}
