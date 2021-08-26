use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let mut ret = -1;
    let all_visited_bit = (1 << graph.len()) - 1;
    let mut dp: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..graph.len() {
      let cur = 1 << i;
      dp.insert(cur, vec![-1;graph.len()]);
      dp.get_mut(&cur).unwrap()[i] = 0;
      let mut stack: Vec<(usize, i32)> = vec![(i, cur)];
      while !stack.is_empty() {
        if let Some((i, cur)) = stack.pop() {
          let step = dp.get(&cur).unwrap()[i];
          for &next in &graph[i] {
            let next_cur = cur | (1 << next);
            if let Some(step_count) = dp.get(&next_cur) {
              if step_count[next as usize] != -1 && step_count[next as usize] <= step + 1 {
                continue
              }
            } else {
              dp.insert(next_cur, vec![-1;graph.len()]);
            }
            dp.get_mut(&next_cur).unwrap()[next as usize] = step + 1;
            stack.push((next as usize, next_cur));
          }
        }
      }
    }
    if let Some(step_count) = dp.get(&all_visited_bit) {
      for i in 0..graph.len() {
        if step_count[i] != -1 && (ret == -1 || step_count[i] < ret) {
          ret = step_count[i];
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    graph: Vec<&'a [i32]>,
    ret: i32
  }

  #[test]
  fn test_shortest_path_length_simple() {
    let suites = vec![
      Suite {
        graph: vec![&[1,2,3],&[0],&[0],&[0]],
        ret: 4
      },
      Suite {
        graph: vec![&[1],&[0,2,4],&[1,3,4],&[2],&[1,2]],
        ret: 4
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::shortest_path_length(Solution::t2_i(s.graph)));
    }
  }
}