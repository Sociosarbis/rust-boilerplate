use super::*;


impl Solution {
  pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut temp = vec![];
    Solution::all_paths_source_target_dfs(&graph, 0, &mut temp, &mut ret);
    ret
  }

  fn all_paths_source_target_dfs(graph: &Vec<Vec<i32>>, i: i32, temp: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    temp.push(i);
    for &item in &graph[i as usize] {
      Solution::all_paths_source_target_dfs(graph, item, temp, out);
    }
    if i as usize == graph.len() - 1 {
      out.push(temp.to_owned());
    }
    temp.pop();
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    graph: Vec<Vec<i32>>,
    ret: Vec<Vec<i32>>
  }

  #[test]
  fn test_all_paths_source_target_simple() {
    let suites = vec![
      Suite {
        graph: t2_i![[1,2],[3],[3],[]],
        ret: t2_i![[0,1,3],[0,2,3]]
      },
      Suite {
        graph: t2_i![[4,3,1],[3,2,4],[3],[4],[]],
        ret: t2_i![[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
      },
      Suite {
        graph: t2_i![[1],[]],
        ret: t2_i![[0,1]]
      },
      Suite {
        graph: t2_i![[1,2,3],[2],[3],[]],
        ret: t2_i![[0,1,2,3],[0,2,3],[0,3]]
      },
      Suite {
        graph: t2_i![[1,3],[2],[3],[]],
        ret: t2_i![[0,1,2,3],[0,3]]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::all_paths_source_target(s.graph));
    }
  }
}