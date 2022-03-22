use super::*;

impl Solution {
  pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut zero_dep_nodes = vec![];
    let mut node_to_dep_count = vec![0;graph.len()];
    let mut node_to_dependants = vec![vec![];graph.len()];
    let mut ret = vec![];
    for i in 0..graph.len() {
      if graph[i].is_empty() {
        zero_dep_nodes.push(i);
      }
      node_to_dep_count[i] = graph[i].len();
      for &node in &graph[i] {
        node_to_dependants[node as usize].push(i);
      }
    }
    while !zero_dep_nodes.is_empty() {
      let n = zero_dep_nodes.len();
      for i in 0..n {
        let node = zero_dep_nodes[i];
        for &j in &node_to_dependants[node] {
          node_to_dep_count[j] -= 1;
          if node_to_dep_count[j] == 0 {
            zero_dep_nodes.push(j);
          }
        }
      }
      zero_dep_nodes.drain(0..n);
    }
    for i in 0..node_to_dep_count.len() {
      if node_to_dep_count[i] == 0 {
        ret.push(i as i32);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    graph: Vec<Vec<i32>>,
    ret: Vec<i32>
  }

  #[test]
  fn test_eventual_safe_nodes_simple() {
    let suites = vec![
      Suite {
        graph: t2_i![[1,2],[2,3],[5],[0],[5],[],[]],
        ret: vec![2,4,5,6]
      },
      Suite {
        graph: t2_i![[1,2,3,4],[1,2],[3,4],[0,4],[]],
        ret: vec![4]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::eventual_safe_nodes(s.graph));
    }
  }
}