use super::*;

impl Solution {
  pub fn is_robot_bounded(instructions: String) -> bool {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut pos = (0, 0);
    let mut index = 0;
    for ch in instructions.chars() {
      match ch {
          'G' => {
            pos.0 += dirs[index].0;
            pos.1 += dirs[index].1;
          }
          'L' => {
            index = (index + 3) % 4;
          }
          'R' => {
            index = (index + 1) % 4;
          }
          _ => unreachable!()
      }
    }
    let pos_diff = pos;
    let index_diff = index;
    while index != 0 {
      match index {
        1 => {
          pos.0 += pos_diff.1;
          pos.1 -= pos_diff.0;
        }
        2 => {
          pos.0 -= pos_diff.0;
          pos.1 -= pos_diff.1;
        }
        3 => {
          pos.0 -= pos_diff.1;
          pos.1 += pos_diff.0;
        }
        _ => unreachable!()
      }
      index = (index + index_diff) % 4;
    }
    pos.0 == 0 && pos.1 == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    instructions: String,
    ret: bool
  }

  #[test]
  fn test_is_robot_bounded_simple() {
    let suites = vec![
      Suite {
        instructions: "GGLLGG".to_string(),
        ret: true
      },
      Suite {
        instructions: "GG".to_string(),
        ret: false
      },
      Suite {
        instructions: "GLG".to_string(),
        ret: true
      },
      Suite {
        instructions: "LLGRL".to_string(),
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_robot_bounded(s.instructions));
    }
  }
}