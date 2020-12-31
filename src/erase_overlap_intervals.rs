use super::Solution;

use std::cmp::Ordering;

impl Solution {
  pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
      if intervals.len() == 0 {
        return 0;
      }
      intervals.sort_unstable_by(|a, b| {
        if a[0] < b[0] || (a[0] == b[0] && a[1] < b[1]) {
          Ordering::Less
        } else {
          Ordering::Greater
        }
      });
      let mut ret = 0;
      let mut prev = &intervals[0];
      for i in 1..intervals.len() {
        if intervals[i][0] >= prev[1] {
          prev = &intervals[i];
        } else {
          if intervals[i][1] < prev[1] {
            prev = &intervals[i];
          }
          ret += 1;
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
    ret: i32
  }
  #[test]
  fn erase_overlap_intervals_simple() {
    let suites = vec![
      Suite {
        intervals: vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]],
        ret: 1
      },
      Suite {
        intervals: vec![vec![1,2],vec![1,2],vec![1,2]],
        ret: 2
      },
      Suite {
        intervals: vec![vec![1,2],vec![2,3]],
        ret: 0
      },
      Suite {
        intervals: vec![vec![1,2],vec![2,8],vec![5,6],vec![7, 8], vec![3,4]],
        ret: 1
      },
    ];
    for s in suites {
      assert_eq!(Solution::erase_overlap_intervals(s.intervals), s.ret);
    }
  }
}