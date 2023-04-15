use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ret = vec![0;n as usize];
    for p in paths {
      if let Some(s) = graph.get_mut(&p[0]) {
        s.push(p[1]);
      } else {
        graph.insert(p[0], vec![p[1]]);
      }

      if let Some(s) = graph.get_mut(&p[1]) {
        s.push(p[0]);
      } else {
        graph.insert(p[1], vec![p[0]]);
      }
    }
    for i in 0..ret.len() {
      let index = i as i32 + 1;
      let mut mask = 15;
      if let Some(neighbors) = graph.get(&index) {
        for &neighbor in neighbors {
          let j = neighbor as usize - 1;
          if ret[j] != 0 {
            let v = 1 << (ret[j] - 1);
            if mask & v != 0 {
              mask -= v;
            }
          }
        }
      }
      for j in 0..4 {
        if mask & (1 << j) != 0 {
          ret[i] = j + 1;
          break;
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
    paths: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_garden_no_adj_simple() {
    let suites = vec![
      Suite {
        n: 3,
        paths: t2_i![[1,2],[2,3],[3,1]],
        ret: vec![1,2,3]
      },
      Suite {
        n: 4,
        paths: t2_i![[1,2],[3,4]],
        ret: vec![1,2,1,2]
      },
      Suite {
        n: 4,
        paths: t2_i![[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]],
        ret: vec![1,2,3,4]
      },
      Suite {
        n: 5,
        paths: t2_i![[3,4],[4,5],[3,2],[5,1],[1,3],[4,2]],
        ret: vec![1,1,2,3,2]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::garden_no_adj(s.n, s.paths));
    }
  }
}