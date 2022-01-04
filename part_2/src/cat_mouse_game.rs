use super::*;

impl Solution {
  pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    Solution::cat_mouse_game_dfs(&graph, 0, 1, 2, &mut vec![vec![vec![-1;graph.len()];graph.len()];graph.len() * 2])
  }

  pub fn cat_mouse_game_dfs(graph: &Vec<Vec<i32>>, t: usize, x: usize, y: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if t == graph.len() * 2 { return 0; }
    if x == y {
      dp[t][x][y] = 2;
    } else if x == 0 {
      dp[t][x][y] = 1;
    }
    if dp[t][x][y] == -1 {
      let mouse_turn = t & 1 == 0;
      if mouse_turn {
        let mut cat_win = true;
        for &item in &graph[x] {
          match Solution::cat_mouse_game_dfs(graph, t + 1, item as usize, y, dp) {
            1 => {
              dp[t][x][y] = 1;
              return dp[t][x][y];
            },
            0 => {
              cat_win = false;
            },
            _ => {}
          }
        }
        dp[t][x][y] = if cat_win { 2 } else { 0 };
      } else {
        let mut mouse_win = true;
        for &item in &graph[y] {
          if item == 0 {
            continue;
          }
          match Solution::cat_mouse_game_dfs(graph, t + 1, x, item as usize, dp) {
            2 => {
              dp[t][x][y] = 2;
              return dp[t][x][y];
            },
            0 => {
              mouse_win = false;
            },
            _ => {}
          }
        }
        dp[t][x][y] = if mouse_win { 1 } else { 0 };
      }
    }
    dp[t][x][y]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    graph: Vec<Vec<i32>>,
    ret: i32
  }

  #[test]
  fn test_cat_mouse_game_simple() {
    let suites = vec![
      Suite {
        graph: t2_i![[2,5],[3],[0,4,5],[1,4,5],[2,3],[0,2,3]],
        ret: 0
      },
      Suite {
        graph: t2_i![[1,3],[0],[3],[0,2]],
        ret: 1
      },
      Suite {
        graph: t2_i![[2,5],[3],[0,4,5],[1,5],[2],[0,2,3]],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::cat_mouse_game(s.graph));
    }
  }
}