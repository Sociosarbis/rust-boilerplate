use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let queen_set: HashSet<Vec<i32>> = queens.into_iter().collect();
    let mut ret = Vec::with_capacity(8);
    for i in -1..=1 {
      'a: for j in -1..=1 {
        if i != 0 || j != 0 {
          let mut pos = vec![king[0] + i, king[1] + j];
          while pos[0] >= 0 && pos[0] < 8 && pos[1] >= 0 && pos[1] < 8 {
            if queen_set.contains(&pos) {
              ret.push(pos);
              continue 'a;
            }
            pos[0] += i;
            pos[1] += j;
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
    queens: Vec<Vec<i32>>,
    king: Vec<i32>,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_queens_attackthe_king_simple() {
    let suites = vec![
      Suite {
        queens: t2_i![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]],
        king: vec![0, 0],
        ret: t2_i![[0, 1], [1, 0], [3, 3]],
      },
      Suite {
        queens: t2_i![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],
        king: vec![3, 3],
        ret: t2_i![[2, 2], [3, 4], [4, 4]],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::queens_attackthe_king(s.queens, s.king));
    }
  }
}
