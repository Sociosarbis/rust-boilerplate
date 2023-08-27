use super::*;

use std::cmp::Ordering;

impl Solution {
  pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    intervals.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
      Ordering::Equal => a[1].cmp(&b[1]),
      res => res,
    });
    ret.push(intervals[0].clone());
    for interval in intervals.into_iter().skip(1) {
      if let Some(mut item) = ret.pop() {
        if interval[0] <= item[1] {
          item[1] = item[1].max(interval[1]);
          ret.push(item);
        } else {
          ret.push(item);
          ret.push(interval);
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
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_merge_simple() {
    let suites = vec![
      Suite {
        intervals: t2_i![[1, 3], [2, 6], [8, 10], [15, 18]],
        ret: t2_i![[1, 6], [8, 10], [15, 18]],
      },
      Suite {
        intervals: t2_i![[1, 4], [4, 5]],
        ret: t2_i![[1, 5]],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::merge(s.intervals));
    }
  }
}
