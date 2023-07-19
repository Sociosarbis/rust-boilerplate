use super::*;

use std::collections::HashSet;

static DIRS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

impl Solution {
  pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut map = HashSet::with_capacity(obstacles.len());
    for o in obstacles {
      map.insert((o[0], o[1]));
    }
    let mut ret = 0;
    let mut pos = [0, 0];
    let mut dir = 0;
    for command in commands {
      match command {
        -2 => {
          dir = (dir + 3) % 4;
        }
        -1 => {
          dir = (dir + 1) % 4;
        }
        s => {
          for _ in 1..=s {
            pos[0] += DIRS[dir][0];
            pos[1] += DIRS[dir][1];
            if map.contains(&(pos[0], pos[1])) {
              pos[0] -= DIRS[dir][0];
              pos[1] -= DIRS[dir][1];
              break;
            }
          }
          let temp = pos[0] * pos[0] + pos[1] * pos[1];
          if temp > ret {
            ret = temp;
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
    commands: Vec<i32>,
    obstacles: Vec<Vec<i32>>,
    ret: i32,
  }

  #[test]
  fn test_robot_sim_simple() {
    let suites = vec![
      Suite {
        commands: vec![4, -1, 3],
        obstacles: vec![],
        ret: 25,
      },
      Suite {
        commands: vec![4, -1, 4, -2, 4],
        obstacles: t2_i![[2, 4]],
        ret: 65,
      },
      Suite {
        commands: vec![6, -1, -1, 6],
        obstacles: vec![],
        ret: 36,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::robot_sim(s.commands, s.obstacles));
    }
  }
}
