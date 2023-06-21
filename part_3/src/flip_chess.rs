use super::*;

static STEPS: [(i32, i32); 8] = [
  (-1, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
  (1, 0),
  (1, -1),
  (0, -1),
  (-1, -1),
];

impl Solution {
  pub fn flip_chess(chessboard: Vec<String>) -> i32 {
    let board: Vec<Vec<char>> = chessboard
      .into_iter()
      .map(|r| r.chars().collect())
      .collect();
    board
      .iter()
      .enumerate()
      .map(|(i, row)| {
        return row
          .iter()
          .enumerate()
          .map(|(j, _)| {
            if board[i][j] == '.' {
              let mut temp_board = board.clone();
              temp_board[i][j] = 'X';
              Solution::flip_chess_count(&mut temp_board, i as i32, j as i32)
            } else {
              0
            }
          })
          .max()
          .unwrap();
      })
      .max()
      .unwrap()
  }

  fn flip_chess_count(board: &mut Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut ret = 0;
    let mut temp = 0;
    let m = board.len() as i32;
    let n = board[0].len() as i32;
    for &step in &STEPS {
      let mut x = i;
      let mut y = j;
      x += step.0;
      y += step.1;
      while x >= 0 && x < m && y >= 0 && y < n {
        match board[x as usize][y as usize] {
          'O' => {
            temp += 1;
          }
          'X' => {
            ret += temp;
            let prev_x = x;
            let prev_y = y;
            while x != i || y != j {
              x -= step.0;
              y -= step.1;
              board[x as usize][y as usize] = 'X';
            }
            x += step.0;
            y += step.1;
            while x != prev_x || y != prev_y {
              ret += Solution::flip_chess_count(board, x, y);
              x += step.0;
              y += step.1;
            }
            break;
          }
          _ => {
            break;
          }
        }
        x += step.0;
        y += step.1;
      }
      temp = 0;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    chessboard: Vec<String>,
    ret: i32,
  }

  #[test]
  fn test_flip_chess_simple() {
    let suites = vec![
      Suite {
        chessboard: t1!["....X.", "....X.", "XOOO..", "......", "......"],
        ret: 3,
      },
      Suite {
        chessboard: t1![".X.", ".O.", "XO."],
        ret: 2,
      },
      Suite {
        chessboard: t1![
          ".......", ".......", ".......", "X......", ".O.....", "..O....", "....OOX"
        ],
        ret: 4,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::flip_chess(s.chessboard));
    }
  }
}
