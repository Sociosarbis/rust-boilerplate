use super::*;

use std::collections::{HashMap, HashSet};

impl Solution {
  pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rows: HashMap<i32, i32> = HashMap::new();
    let mut cols: HashMap<i32, i32> = HashMap::new();
    let mut slashes: HashMap<i32, i32> = HashMap::new();
    let mut back_slashes: HashMap<i32, i32> = HashMap::new();
    let mut lamp_set: HashSet<(i32, i32)> = HashSet::new();
    let mut ret = vec![0;queries.len()];
    for lamp in &lamps {
      if !lamp_set.contains(&(lamp[0], lamp[1])) {
        lamp_set.insert((lamp[0], lamp[1]));
        *rows.entry(lamp[0]).or_insert(0) += 1;
        *cols.entry(lamp[1]).or_insert(0) += 1;
        *slashes.entry(lamp[1] + n - 1 - lamp[0]).or_insert(0) += 1;
        *back_slashes.entry(lamp[1] + lamp[0]).or_insert(0) += 1;
      }
    }
    for i in 0..queries.len() {
      let query = &queries[i];
      if rows.contains_key(&query[0]) || cols.contains_key(&query[1]) || slashes.contains_key(&(query[1] + n - 1 - query[0])) || back_slashes.contains_key(&(query[1] + query[0])) {
        ret[i] = 1;
        for i in -1..2 {
          for j in -1..2 {
            let coord = (query[0] + i, query[1] + j);
            if lamp_set.contains(&coord) {
              lamp_set.remove(&coord);
              let mut key = coord.0;
              if rows.contains_key(&key) {
                let item = rows.get_mut(&key).unwrap();
                *item -= 1;
                if *item == 0 {
                  rows.remove(&key);
                }
              }
              key = coord.1;
              if cols.contains_key(&key) {
                let item = cols.get_mut(&key).unwrap();
                *item -= 1;
                if *item == 0 {
                  cols.remove(&key);
                }
              }
              key = coord.1 + n - 1 - coord.0;
              if slashes.contains_key(&key) {
                let item = slashes.get_mut(&key).unwrap();
                *item -= 1;
                if *item == 0 {
                  slashes.remove(&key);
                }
              }
              key = coord.1 + coord.0;
              if back_slashes.contains_key(&key) {
                let item = back_slashes.get_mut(&key).unwrap();
                *item -= 1;
                if *item == 0 {
                  back_slashes.remove(&key);
                }
              }
            }
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
    n: i32,
    lamps: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_grid_illumination_simple() {
    let suites = vec![
      Suite {
        n: 5,
        lamps: t2_i![[0,0],[4,4]],
        queries: t2_i![[1,1],[1,0]],
        ret: vec![1,0]
      },
      Suite {
        n: 5,
        lamps: t2_i![[0,0],[4,4]],
        queries: t2_i![[1,1],[1,1]],
        ret: vec![1,1]
      },
      Suite {
        n: 5,
        lamps: t2_i![[0,0],[0,4]],
        queries: t2_i![[0,4],[0,1],[1,4]],
        ret: vec![1,1,0]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::grid_illumination(s.n, s.lamps, s.queries));
    }
  }
}