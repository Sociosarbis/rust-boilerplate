use super::*;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
  pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut pq: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut ret = vec![-1; queries.len()];
    intervals.sort_unstable();
    let mut queries: Vec<(i32, usize)> = queries
      .into_iter()
      .enumerate()
      .map(|(i, v)| (v, i))
      .collect();
    queries.sort_unstable();
    let mut j = 0;
    for q in queries {
      while j < intervals.len() {
        if intervals[j][0] <= q.0 {
          if intervals[j][1] >= q.0 {
            pq.push((
              Reverse(intervals[j][1] - intervals[j][0] + 1),
              intervals[j][0],
            ));
          }
          j += 1;
        } else {
          break;
        }
      }
      while let Some((Reverse(v), l)) = pq.pop() {
        if q.0 <= l + v - 1 {
          ret[q.1] = v;
          pq.push((Reverse(v), l));
          break;
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
    intervals: Vec<Vec<i32>>,
    queries: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_min_interval_simple() {
    let suites = vec![
      Suite {
        intervals: t2_i![[1, 4], [2, 4], [3, 6], [4, 4]],
        queries: vec![2, 3, 4, 5],
        ret: vec![3, 3, 1, 4],
      },
      Suite {
        intervals: t2_i![[2, 3], [2, 5], [1, 8], [20, 25]],
        queries: vec![2, 19, 5, 22],
        ret: vec![2, -1, 4, 6],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_interval(s.intervals, s.queries));
    }
  }
}
