use super::*;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
  pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let mut neighbors = vec![vec![]; n as usize];
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![false; n as usize]; n as usize];
    for e in edges {
      if e[2] <= distance_threshold {
        neighbors[e[0] as usize].push((e[1], e[2]));
        heap.push((Reverse(e[2]), e[0], e[1]));
        neighbors[e[1] as usize].push((e[0], e[2]));
        heap.push((Reverse(e[2]), e[1], e[0]));
      }
    }
    let mut counts = vec![0; n as usize];

    while let Some((Reverse(dist), from, to)) = heap.pop() {
      if !visited[from as usize][to as usize] {
        visited[from as usize][to as usize] = true;
        counts[from as usize] += 1;
        for &neighbor in &neighbors[to as usize] {
          if from != neighbor.0
            && neighbor.1 + dist <= distance_threshold
            && !visited[from as usize][neighbor.0 as usize]
          {
            heap.push((Reverse(neighbor.1 + dist), from, neighbor.0));
          }
        }
      }
    }

    counts
      .into_iter()
      .enumerate()
      .fold((n, n as usize), |acc, (i, count)| {
        if count <= acc.0 {
          (count, i)
        } else {
          acc
        }
      })
      .1 as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    edges: Vec<Vec<i32>>,
    distance_threshold: i32,
    ret: i32,
  }

  #[test]
  fn test_find_the_city_simple() {
    let suites = vec![
      Suite {
        n: 4,
        edges: t2_i![[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]],
        distance_threshold: 4,
        ret: 3,
      },
      Suite {
        n: 5,
        edges: t2_i![
          [0, 1, 2],
          [0, 4, 8],
          [1, 2, 3],
          [1, 4, 2],
          [2, 3, 1],
          [3, 4, 1]
        ],
        distance_threshold: 2,
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::find_the_city(s.n, s.edges, s.distance_threshold)
      );
    }
  }
}
