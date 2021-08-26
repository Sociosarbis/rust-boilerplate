use super::*;
use std::cmp::max;

impl Solution {
  pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let m = heights.len();
    let n = heights[0].len();
    let mut dp:Vec<Vec<i32>> = vec![vec![-1;n];m];
    dp[0][0] = 0;
    let mut bfs: Vec<(usize, usize)> = vec![];
    bfs.push((0, 0));
    let dirs: Vec<[i32;2]> = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];
    while !bfs.is_empty() {
      if let Some((i, j)) = bfs.pop() {
        let cur_height = heights[i][j];
        for d in &dirs {
          let next_i = i as i32 + d[0];
          let next_j = j as i32 + d[1];
          if next_i >= 0 && next_i < m as i32 && next_j >= 0 && next_j < n  as i32 {
            let diff = max(dp[i][j], (heights[next_i as usize][next_j as usize] - cur_height).abs());
            let prev_diff = dp[next_i as usize][next_j as usize];
            if prev_diff == -1 || diff < prev_diff {
              dp[next_i as usize][next_j as usize] = diff;
              bfs.push((next_i as usize, next_j as usize));
            }
          }
        }
      }
    }
    return dp[m - 1][n - 1];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    heights: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn minimum_effort_path_simple() {
    let suites = vec![
      Suite {
        heights: Solution::t2_i(vec![&[1,2,2],&[3,8,2],&[5,3,5]]),
        ret: 2
      },
      Suite {
        heights: Solution::t2_i(vec![&[1,2,3],&[3,8,4],&[5,3,5]]),
        ret: 1
      },
      Suite {
        heights: Solution::t2_i(vec![&[1,2,1,1,1],&[1,2,1,2,1],&[1,2,1,2,1],&[1,2,1,2,1],&[1,1,1,2,1]]),
        ret: 0
      },
      Suite {
        heights: Solution::t2_i(vec![&[1,10,6,7,9,10,4,9]]),
        ret: 9
      }
    ];

    for s in suites {
      assert_eq!(Solution::minimum_effort_path(s.heights), s.ret);
    }
  }
}