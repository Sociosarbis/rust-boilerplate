use super::*;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl Solution {
  pub fn modified_graph_edges(
    n: i32,
    mut edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
  ) -> Vec<Vec<i32>> {
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n as usize];
    for (i, edge) in edges.iter().enumerate() {
      let a = edge[0] as usize;
      let b = edge[1] as usize;
      if edge[2] != -1 {
        graph[a].push((b, i));
        graph[b].push((a, i));
      }
    }
    let mut visited = vec![false; n as usize];
    visited[source as usize] = true;
    let res = Solution::modified_graph_edges_dfs(
      source as usize,
      destination as usize,
      target,
      &edges,
      &graph,
    );
    match res.cmp(&target) {
      Ordering::Less => {
        vec![]
      }
      Ordering::Equal => {
        for edge in &mut edges {
          if edge[2] == -1 {
            edge[2] = target;
          }
        }
        edges
      }
      Ordering::Greater => {
        for i in 0..edges.len() {
          if edges[i][2] == -1 {
            let a = edges[i][0] as usize;
            let b = edges[i][1] as usize;
            edges[i][2] = 1;
            graph[a].push((b, i));
            graph[b].push((a, i));
            let res = Solution::modified_graph_edges_dfs(
              source as usize,
              destination as usize,
              target,
              &edges,
              &graph,
            );
            if res <= target {
              edges[i][2] += target - res;
              for edge in edges.iter_mut().skip(i + 1) {
                if edge[2] == -1 {
                  edge[2] = target;
                }
              }
              return edges;
            }
          }
        }
        vec![]
      }
    }
  }

  // 需多次使用dijkstra算法
  #[allow(clippy::too_many_arguments)]
  fn modified_graph_edges_dfs(
    source: usize,
    destination: usize,
    target: i32,
    edges: &[Vec<i32>],
    graph: &[Vec<(usize, usize)>],
  ) -> i32 {
    let mut dist = vec![target + 1; graph.len()];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), source));
    dist[source] = 0;
    while let Some((Reverse(d), i)) = heap.pop() {
      for &(j, edge_idx) in &graph[i] {
        let temp = d + edges[edge_idx][2];
        if temp < dist[j] {
          dist[j] = temp;
          heap.push((Reverse(temp), j));
        }
      }
    }
    dist[destination]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    target: i32,
    ret: Vec<Vec<i32>>,
  }

  #[test]
  fn test_modified_graph_edges_simple() {
    let suites = vec![
      // Suite {
      //   n: 5,
      //   edges: t2_i![[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]],
      //   source: 0,
      //   destination: 1,
      //   target: 5,
      //   ret: t2_i![[4, 1, 1], [2, 0, 1], [0, 3, 3], [4, 3, 1]],
      // },
      // Suite {
      //   n: 3,
      //   edges: t2_i![[0, 1, -1], [0, 2, 5]],
      //   source: 0,
      //   destination: 2,
      //   target: 6,
      //   ret: vec![],
      // },
      Suite {
        n: 4,
        edges: t2_i![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]],
        source: 0,
        destination: 2,
        target: 6,
        ret: t2_i![[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]],
      },
      Suite {
        n: 5,
        edges: t2_i![
          [1, 4, 1],
          [2, 4, -1],
          [3, 0, 2],
          [0, 4, -1],
          [1, 3, 10],
          [1, 0, 10]
        ],
        source: 0,
        destination: 2,
        target: 15,
        ret: t2_i![
          [1, 4, 1],
          [2, 4, 4],
          [3, 0, 2],
          [0, 4, 14],
          [1, 3, 10],
          [1, 0, 10]
        ],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::modified_graph_edges(s.n, s.edges, s.source, s.destination, s.target)
      );
    }
  }
}
