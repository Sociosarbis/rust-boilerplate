use super::*;

impl Solution {
  pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let mut x = 0;
    let mut o = 0;
    let board_c: Vec<Vec<char>> = board.iter().map(|item| item.chars().collect()).collect();
    for s in board {
      for c in s.chars() {
        match c {
          'X' => x += 1,
          'O' => o += 1,
          _ => {}
        }
      }
    }
    if x < o || x > o + 1 {
      return false;
    }
    if x >= 3 {
      let mut flag = false;
      for i in 0..3 {
        let c = board_c[i][0];
        if c != ' ' {
          let mut count = 1;
          for j in 1..3 {
            if board_c[i][j] == c {
              count += 1;
            } else {
              break;
            }
          }
          if count == 3 {
            if flag || (x == o && c == 'X') || (x > o && c == 'O') {
              return false;
            } else {
              flag = true;
            }
          }
        }
      }
      flag = false;
      for i in 0..3 {
        let c = board_c[0][i];
        if c != ' ' {
          let mut count = 1;
          for j in 1..3 {
            if board_c[j][i] == c {
              count += 1;
            } else {
              break;
            }
          }
          if count == 3 {
            if flag || (x == o && c == 'X') || (x > o && c == 'O') {
              return false;
            } else {
              flag = true;
            }
          }
        }
      }
      {
        let c = board_c[0][0];
        if c != ' ' {
          let mut count = 1;
          for j in 1..3 {
            if board_c[j][j] == c {
              count += 1;
            } else {
              break;
            }
          }
          if count == 3 && ((x == o && c == 'X') || (x > o && c == 'O')) {
            return false;
          }
        }
      }
      {
        let c = board_c[0][2];
        if c != ' ' {
          let mut count = 1;
          for j in 1..3 {
            if board_c[j][2 - j] == c {
              count += 1;
            } else {
              break;
            }
          }
          if count == 3 && ((x == o && c == 'X') || (x > o && c == 'O')) {
            return false;
          }
        }
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    board: Vec<String>,
    ret: bool
  }

  #[test]
  fn test_valid_tic_tac_toe_simple() {
    let suites = vec![
      Suite {
        board: t1!["O  ","   ","   "],
        ret: false
      },
      Suite {
        board: t1!["XOX"," X ","   "],
        ret: false
      },
      Suite {
        board: t1!["XOX","O O","XOX"],
        ret: true
      },
      Suite {
        board: t1!["XXX","OOX","OOX"],
        ret: true
      },
      Suite {
        board: t1!["OXX","XOX","OXO"],
        ret: false
      },
      Suite {
        board: t1!["XXX","XOO","OO "],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::valid_tic_tac_toe(s.board));
    }
  }
}