use super::*;
use std::collections::{HashMap, VecDeque};


impl Solution {
  pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in 0..arr.len() {
      let item = arr[i];
      if !map.contains_key(&item) {
        map.insert(item, vec![]);
      }
      if !((i + 1 < arr.len() && arr[i] == arr[i + 1]) && (i > 0 && arr[i] == arr[i - 1])) {
        map.get_mut(&item).unwrap().push(i);
      }
    }
    let mut visited = vec![false;arr.len()];
    visited[0] = true;
    let mut bfs = VecDeque::new();
    bfs.push_back(0);
    let mut ret = 0;
    while !bfs.is_empty() {
      let n = bfs.len();
      for _ in 0..n {
        let index = bfs.pop_front().unwrap();
        if index + 1 == arr.len() {
          return ret;
        }
        if index > 0 && arr[index] != arr[index - 1] && !visited[index - 1] {
          visited[index - 1] = true;
          bfs.push_back(index - 1);
        }
        if index + 1 < arr.len() && arr[index] != arr[index + 1] && !visited[index + 1] {
          visited[index + 1] = true;
          bfs.push_back(index + 1);
        }
        for indices in map.get(&arr[index]) {
          for &i in indices {
            if !visited[i] {
              visited[i] = true;
              bfs.push_back(i);
            }
          }
        }
        map.remove(&arr[index]);
      }
      ret += 1;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_min_jumps_simple() {
    let suites = vec![
      Suite {
        arr: vec![100,-23,-23,404,100,23,23,23,3,404],
        ret: 3
      },
      Suite {
        arr: vec![7],
        ret: 0
      },
      Suite {
        arr: vec![7,6,9,6,9,6,9,7],
        ret: 1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_jumps(s.arr));
    }
  }
}