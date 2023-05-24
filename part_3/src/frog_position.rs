use super::*;

impl Solution {
  pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    let mut visited = vec![false; n as usize + 1];
    let mut neighbours: Vec<Vec<usize>> = vec![vec![]; n as usize + 1];
    for edge in edges {
      neighbours[edge[0] as usize].push(edge[1] as usize);
      neighbours[edge[1] as usize].push(edge[0] as usize)
    }
    visited[1] = true;
    Solution::frog_position_dfs(t, 1, 1.0, target as usize, &neighbours, &mut visited)
  }

  pub fn frog_position_dfs(
    rest_time: i32,
    cur_pos: usize,
    p: f64,
    target: usize,
    neighbours: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
  ) -> f64 {
    if rest_time == 0 {
      if cur_pos == target {
        return p;
      } else {
        return 0.0;
      }
    }
    let mut ret = 0.0;
    let options = neighbours[cur_pos]
      .iter()
      .fold(0, |acc, &next| acc + if !visited[next] { 1 } else { 0 });
    if cur_pos == target {
      if options == 0 {
        return p;
      } else {
        return 0.0;
      }
    }
    for &neighbour in &neighbours[cur_pos] {
      if !visited[neighbour] {
        visited[neighbour] = true;
        ret += Solution::frog_position_dfs(
          rest_time - 1,
          neighbour,
          p / options as f64,
          target,
          neighbours,
          visited,
        );
        visited[neighbour] = false;
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
    t: i32,
    target: i32,
    ret: f64,
  }

  #[test]
  fn test_frog_position_simple() {
    let suites = vec![
      Suite {
        n: 7,
        edges: t2_i![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
        t: 2,
        target: 4,
        ret: 0.16666666666666666,
      },
      Suite {
        n: 7,
        edges: t2_i![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
        t: 1,
        target: 7,
        ret: 0.3333333333333333,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::frog_position(s.n, s.edges, s.t, s.target));
    }
  }
}
