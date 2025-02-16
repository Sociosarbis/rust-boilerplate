use super::*;

#[derive(Copy, Clone)]
enum Res {
  Tie,
  MouseWin,
  CatWin,
}

#[derive(Clone, Copy)]
enum Turn {
  Mouse,
  Cat,
}

const HOLE: usize = 0;

impl Solution {
  pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    // 从确定的状态，回推未确定的状态
    // degree表示该状态下可移动的可选项
    let mut dp = vec![vec![[Res::Tie; 2]; n]; n];
    let mut degree = vec![vec![[0; 2]; n]; n];
    let on_finish = |dp: &mut Vec<Vec<[Res; 2]>>,
                     queue: &mut Vec<(usize, usize, Turn)>,
                     i: usize,
                     j: usize,
                     t: Turn,
                     r: Res| {
      dp[i][j][t as usize] = r;
      queue.push((i, j, t));
    };
    for i in 0..n {
      for j in 0..n {
        degree[i][j][Turn::Mouse as usize] = graph[i].len();
        degree[i][j][Turn::Cat as usize] = graph[j].len();
      }
      for &j in &graph[HOLE] {
        degree[i][j as usize][Turn::Cat as usize] -= 1;
      }
    }
    let mut queue: Vec<(usize, usize, Turn)> = vec![];
    for j in 1..n {
      on_finish(&mut dp, &mut queue, HOLE, j, Turn::Mouse, Res::MouseWin);
      on_finish(&mut dp, &mut queue, HOLE, j, Turn::Cat, Res::MouseWin);
    }
    for i in 1..n {
      on_finish(&mut dp, &mut queue, i, i, Turn::Mouse, Res::CatWin);
      on_finish(&mut dp, &mut queue, i, i, Turn::Cat, Res::CatWin);
    }
    while !queue.is_empty() {
      let (i, j, t) = queue.remove(0);
      let prev_states: Vec<(usize, usize)> = match t {
        Turn::Mouse => graph[j]
          .iter()
          .filter_map(|&prev_j| {
            if let Res::Tie = dp[i][prev_j as usize][Turn::Cat as usize] {
              if prev_j as usize == HOLE {
                None
              } else {
                Some((i, prev_j as usize))
              }
            } else {
              None
            }
          })
          .collect(),
        _ => graph[i]
          .iter()
          .filter_map(|&prev_i| {
            if let Res::Tie = dp[prev_i as usize][j][Turn::Mouse as usize] {
              Some((prev_i as usize, j))
            } else {
              None
            }
          })
          .collect(),
      };
      let r = dp[i][j][t as usize];
      let prev_t = if let Turn::Mouse = t {
        Turn::Cat
      } else {
        Turn::Mouse
      };
      for (prev_i, prev_j) in prev_states {
        // 当轮到自己，且能够导向胜利，则一定是这个选项
        match r {
          Res::MouseWin => {
            if let Turn::Mouse = prev_t {
              on_finish(&mut dp, &mut queue, prev_i, prev_j, prev_t, r);
              continue;
            }
          }
          _ => {
            if let Turn::Cat = prev_t {
              on_finish(&mut dp, &mut queue, prev_i, prev_j, prev_t, r);
              continue;
            }
          }
        }
        degree[prev_i][prev_j][prev_t as usize] -= 1;
        // 当不可选时，只能设置为下一步的结果
        if degree[prev_i][prev_j][prev_t as usize] == 0 {
          on_finish(&mut dp, &mut queue, prev_i, prev_j, prev_t, r);
        }
      }
    }
    dp[1][2][Turn::Mouse as usize] as _
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    graph: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_cat_mouse_game_simple() {
    let suites = vec![
      Suite {
        graph: t2_i![[2, 5], [3], [0, 4, 5], [1, 4, 5], [2, 3], [0, 2, 3]],
        ret: 0,
      },
      Suite {
        graph: t2_i![[1, 3], [0], [3], [0, 2]],
        ret: 1,
      },
      Suite {
        graph: t2_i![[2, 5], [3], [0, 4, 5], [1, 5], [2], [0, 2, 3]],
        ret: 2,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::cat_mouse_game(s.graph));
    }
  }
}
