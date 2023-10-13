use super::*;

use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap, HashSet},
};

impl Solution {
  pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![-1; rains.len()];
    let mut rain_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();
    let mut rain_set: HashSet<i32> = HashSet::new();
    let mut heap: BinaryHeap<(Reverse<usize>, i32)> = BinaryHeap::new();
    for (i, &rain) in rains.iter().enumerate().rev() {
      if let Some(v) = rain_to_indices.get_mut(&rain) {
        v.push(i);
      } else {
        rain_to_indices.insert(rain, vec![i]);
      }
    }
    let mut add_num = |num: i32, index: usize| {
      if let Some(v) = rain_to_indices.get_mut(&num) {
        while let Some(last_index) = v.pop() {
          if last_index > index {
            return Some((Reverse(last_index), num));
          }
        }
      }
      None
    };
    'a: for (i, rain) in rains.into_iter().enumerate() {
      if rain == 0 {
        if rain_set.is_empty() {
          ret[i] = 1;
        } else {
          while let Some((Reverse(index), num)) = heap.pop() {
            if index > i {
              ret[i] = num;
              rain_set.remove(&num);
              continue 'a;
            } else {
              if let Some(item) = add_num(num, i) {
                heap.push(item);
              }
            }
          }
          ret[i] = 1;
        }
      } else {
        if rain_set.contains(&rain) {
          return vec![];
        }
        rain_set.insert(rain);
        if let Some(item) = add_num(rain, i) {
          heap.push(item);
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
    rains: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_avoid_flood_simple() {
    let suites = vec![
      Suite {
         rains: vec![1, 2, 3, 4],
         ret: vec![-1, -1, -1, -1],
      },
      Suite {
        rains: vec![1, 2, 0, 0, 2, 1],
        ret: vec![-1, -1, 2, 1, -1, -1],
      },
      Suite {
        rains: vec![1, 2, 0, 1, 2],
        ret: vec![],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::avoid_flood(s.rains));
    }
  }
}
