use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut counter = vec![HashMap::new(); n as usize];
    let mut edge_counter = vec![0; n as usize];
    let mut nodes: Vec<usize> = (0..n).map(|i| i as usize).collect();
    let mut id_to_index = vec![0;n as usize];
    for e in edges {
      let item = (e[0] as usize - 1, e[1] as usize - 1);
      edge_counter[item.0] += 1;
      edge_counter[item.1] += 1;
      if let Some(v) = counter[item.0].get_mut(&item.1) {
        *v += 1;
      } else {
        counter[item.0].insert(item.1, 1);
      }
      if let Some(v) = counter[item.1].get_mut(&item.0) {
        *v += 1;
      } else {
        counter[item.1].insert(item.0, 1);
      }
    }
    nodes.sort_unstable_by(|&a, &b| edge_counter[a].cmp(&edge_counter[b]));
    for (i, &id) in nodes.iter().enumerate() {
      id_to_index[id] = i;
    }
    let mut ret = vec![0; queries.len()];
    for (index, q) in queries.into_iter().enumerate() {
      let mut e = n as usize;
      for i in 0..nodes.len() - 1 {
        if e <= i + 1 {
          e = i + 1;
        }
        let mut r = e;
        let a_id = nodes[i];
        while r > i + 1 {
          let temp = edge_counter[a_id] + edge_counter[nodes[r - 1]];
          if temp > q {
            r -= 1;
            e = r;
          } else {
            break;
          }
        }
        ret[index] += n - r as i32;
        for (&b_id, &c) in &counter[a_id] {
          if id_to_index[b_id] >= r {
            if edge_counter[a_id] + edge_counter[b_id] - c <= q {
              ret[index] -= 1;
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
    edges: Vec<Vec<i32>>,
    queries: Vec<i32>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_count_pairs_simple() {
    let suites = vec![
      Suite {
        n: 4,
        edges: t2_i![[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]],
        queries: vec![2, 3],
        ret: vec![6, 5],
      },
      Suite {
        n: 5,
        edges: t2_i![
          [1, 5],
          [1, 5],
          [3, 4],
          [2, 5],
          [1, 3],
          [5, 1],
          [2, 3],
          [2, 5]
        ],
        queries: vec![1, 2, 3, 4, 5],
        ret: vec![10, 10, 9, 8, 6],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_pairs(s.n, s.edges, s.queries));
    }
  }
}
