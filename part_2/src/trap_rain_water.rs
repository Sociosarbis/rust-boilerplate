use super::*;

use std::collections::BinaryHeap;
use std::cmp::{Ord, PartialOrd, PartialEq, Ordering};

#[derive(Eq)]
struct Item (i32, usize, usize);

impl PartialEq for Item {
  fn eq(&self, other: &Self) -> bool {
    return self.0 == other.0;
  }
}

impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(other.0.cmp(&self.0));
  }
}

impl Ord for Item {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.partial_cmp(other).unwrap();
  }
}

static DIRS: [[i32;2];4] = [[-1,0],[1,0],[0,-1],[0,1]];

impl Solution {
  pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let m = height_map.len();
    let n = height_map[0].len();
    if m < 2 || n < 2 {
      return 0;
    }
    let mut visited = vec![vec![false;n];m];
    visited[0][0] = true;
    visited[0][n - 1] = true;
    visited[m - 1][0] = true;
    visited[m - 1][n - 1] = true;
    let mut min_heap = BinaryHeap::new();
    for i in 1..m - 1 {
      visited[i][0] = true;
      min_heap.push(Item(height_map[i][0], i, 0));
      visited[i][n - 1] = true;
      min_heap.push(Item(height_map[i][n - 1], i, n - 1));
    }
    for i in 1..n - 1 {
      visited[0][i] = true;
      min_heap.push(Item(height_map[0][i], 0, i));
      visited[m - 1][i] = true;
      min_heap.push(Item(height_map[m-1][i], m-1, i));
    }
    let mut ret = 0;
    // 搜索与当前最小的高度的方块相通的方块，如果比它小，增加与高度差相同的水
    // 当把蓄满最小高度的水后，对下一个最小高度进行搜索
    while let Some(cur) = min_heap.pop() {
      for dir in &DIRS {
        let row = cur.1 as i32 + dir[0];
        let col = cur.2 as i32 + dir[1];
        if row >= 0 && row < m as i32 && col >= 0 && col < n as i32 {
          let i = row as usize;
          let j = col as usize;
          if !visited[i][j] {
            if cur.0 >= height_map[i][j] {
              ret += cur.0 - height_map[i][j];
              min_heap.push(Item(cur.0, i, j));
            } else {
              min_heap.push(Item(height_map[i][j], i, j));
            }
            visited[i][j] = true;
          }
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
    height_map: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_trap_rain_water_simple() {
    let suites = vec![
      Suite {
        height_map: t2_i![[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]],
        ret: 4
      },
      Suite {
        height_map: t2_i![[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]],
        ret: 10
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::trap_rain_water(s.height_map));
    }
  }
}