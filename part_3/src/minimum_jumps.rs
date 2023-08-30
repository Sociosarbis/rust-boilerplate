use super::*;

use std::collections::HashSet;

impl Solution {
  pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
    let mut forbidden_set = HashSet::new();
    for pos in forbidden {
      forbidden_set.insert(pos);
    }
    let mut visited = HashSet::new();
    visited.insert((0, false));
    let mut bfs = vec![(0, false)];
    let mut ret = 0;
    while !bfs.is_empty() {
      let n = bfs.len();
      for i in 0..n {
        let pos = bfs[i].0;
        if pos == x {
          return ret;
        }
        if !(pos > x && a >= b) {
          let next_p = (pos + a, false);
          if next_p.0 <= 6000 && !forbidden_set.contains(&next_p.0) && !visited.contains(&next_p) {
            visited.insert(next_p);
            bfs.push(next_p);
          }
        }
        if !bfs[i].1 {
          let next_p = (pos - b, true);
          if next_p.0 > 0
            && !forbidden_set.contains(&next_p.0)
            && !visited.contains(&next_p)
            && !visited.contains(&(next_p.0, false))
          {
            visited.insert(next_p);
            bfs.push(next_p);
          }
        }
      }
      ret += 1;
      bfs.drain(0..n);
    }
    -1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    forbidden: Vec<i32>,
    a: i32,
    b: i32,
    x: i32,
    ret: i32,
  }

  #[test]
  fn test_minimum_jumps_simple() {
    let suites = vec![
      Suite {
        forbidden: vec![14, 4, 18, 1, 15],
        a: 3,
        b: 15,
        x: 9,
        ret: 3,
      },
      Suite {
        forbidden: vec![8, 3, 16, 6, 12, 20],
        a: 15,
        b: 13,
        x: 11,
        ret: -1,
      },
      Suite {
        forbidden: vec![1, 6, 2, 14, 5, 17, 4],
        a: 16,
        b: 9,
        x: 7,
        ret: 2,
      },
      Suite {
        forbidden: vec![
          162, 118, 178, 152, 167, 100, 40, 74, 199, 186, 26, 73, 200, 127, 30, 124, 193, 84, 184,
          36, 103, 149, 153, 9, 54, 154, 133, 95, 45, 198, 79, 157, 64, 122, 59, 71, 48, 177, 82,
          35, 14, 176, 16, 108, 111, 6, 168, 31, 134, 164, 136, 72, 98,
        ],
        a: 29,
        b: 98,
        x: 80,
        ret: 121,
      },
      Suite {
        forbidden: vec![1998],
        a: 1999,
        b: 2000,
        x: 2000,
        ret: 3998,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::minimum_jumps(s.forbidden, s.a, s.b, s.x));
    }
  }
}
