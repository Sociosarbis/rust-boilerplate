use super::*;

impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut groups = [[false;9];27];
    for i in 0..board.len() {
      let row = &board[i];
      for j in 0..row.len() {
        match row[j] {
          '.' => {}
          _ => {
            let index = row[j] as usize - 49;
            let group_indices = [i, 9 + j, 18 + (i / 3) * 3 + (j / 3)];
            for &g in &group_indices {
              if groups[g][index] {
                return false;
              } else {
                groups[g][index] = true;
              }
            }
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
    board: Vec<Vec<char>>,
    ret: bool
  }

  #[test]
  fn test_is_valid_sudoku_simple() {
    let suites = vec![
      Suite {
        board: t2_c![["5","3",".",".","7",".",".",".","."]
        ,["6",".",".","1","9","5",".",".","."]
        ,[".","9","8",".",".",".",".","6","."]
        ,["8",".",".",".","6",".",".",".","3"]
        ,["4",".",".","8",".","3",".",".","1"]
        ,["7",".",".",".","2",".",".",".","6"]
        ,[".","6",".",".",".",".","2","8","."]
        ,[".",".",".","4","1","9",".",".","5"]
        ,[".",".",".",".","8",".",".","7","9"]],
        ret: true
      },
      Suite {
        board: t2_c![["8","3",".",".","7",".",".",".","."]
        ,["6",".",".","1","9","5",".",".","."]
        ,[".","9","8",".",".",".",".","6","."]
        ,["8",".",".",".","6",".",".",".","3"]
        ,["4",".",".","8",".","3",".",".","1"]
        ,["7",".",".",".","2",".",".",".","6"]
        ,[".","6",".",".",".",".","2","8","."]
        ,[".",".",".","4","1","9",".",".","5"]
        ,[".",".",".",".","8",".",".","7","9"]],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_valid_sudoku(s.board));
    }
  }
}