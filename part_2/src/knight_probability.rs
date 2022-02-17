use super::*;

use std::collections::HashMap;

static DIRS: [(i32, i32);8] = [(-2, -1), (-1, -2), (1, -2), (2, -1),(-2, 1), (-1, 2), (1, 2), (2, 1)];

impl Solution {
  pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut map: HashMap<(i32, i32), f64> = HashMap::new();
    map.insert((row, column), 1.0);
    for _ in 0..k {
      let mut new_map = HashMap::new();
      for (item, &count) in &map {
        for dir in &DIRS {
          let pos = (item.0 + dir.0, item.1 + dir.1);
          if pos.0 >= 0 && pos.0 < n && pos.1 >= 0 && pos.1 < n {
            if !new_map.contains_key(&pos) {
              new_map.insert(pos, count);
            } else {
              *new_map.get_mut(&pos).unwrap() += count;
            }
          }
        }
      }
      map = new_map;
    }
    let mut valid = 0.0;
    for (_, &count) in &map {
        valid += count;
    }
    for _ in 0..k {
      valid /= 8.0;
    }
    valid
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    k: i32,
    row: i32,
    column: i32,
    ret: f64
  }

  #[test]
  fn test_knight_probability_simple() {
    let suites = vec![
      Suite {
        n: 3,
        k: 2,
        row: 0,
        column: 0,
        ret: 0.06250
      },
      Suite {
        n: 1,
        k: 0,
        row: 0,
        column: 0,
        ret: 1.00000
      },
      Suite {
        n: 8,
        k: 30,
        row: 6,
        column: 4,
        ret: 0.00019
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::knight_probability(s.n, s.k, s.row, s.column));
    }
  }
}