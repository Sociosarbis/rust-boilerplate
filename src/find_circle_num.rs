use super::Solution;

impl Solution {
  pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let mut visited: Vec<bool> = vec![false;is_connected.len()];
    for i in 0..is_connected.len() {
      ret += Solution::dfs(&is_connected, i, &mut visited);
    }
    ret
  }

  fn dfs(is_connected: &Vec<Vec<i32>>, i: usize, visited: &mut Vec<bool>) -> i32 {
    return if !visited[i] {
      visited[i] = true;
      for j in 0..is_connected[i].len() {
        if is_connected[i][j] == 1 && !visited[j] {
          Solution::dfs(is_connected, j, visited);
        }
      }
      1
    } else { 0 }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    is_connected: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn find_circle_num_simple() {
    let suites: Vec<Suite> = vec![
      Suite {
        is_connected: vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]],
        ret: 2
      },
      Suite {
        is_connected: vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]],
        ret: 3
      }
    ];
    
    for s in suites {
      assert_eq!(Solution::find_circle_num(s.is_connected), s.ret);
    }
  }
}