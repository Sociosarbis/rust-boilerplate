use super::*;
use std::collections::HashMap;

impl Solution {
  pub fn find_min_step(board: String, hand: String) -> i32 {
    let mut counter: HashMap<char, i32> = hand.chars().fold(HashMap::new(), |mut acc, c| {
      if let Some(count) = acc.get_mut(&c) {
        *count += 1;
      } else {
        acc.insert(c, 1);
      }
      acc
    });
    Solution::find_min_step_dfs(&mut board.chars().collect(), 0, &mut counter, 0)
  }

  fn find_min_step_dfs(board: &mut Vec<char>, i: usize, counter: &mut HashMap<char, i32>, step: i32) -> i32 {
    if i >= board.len() {
      return -1;
    }
    let cur = board[i];
    let mut count = 1;
    for j in i + 1..board.len() {
      if board[j] == cur {
        count += 1;
      } else {
        break;
      }
    }
    let mut ret = -1;
    if let Some(&c) = counter.get(&cur) {
      if count + c >= 3 {
        let mut index = i;
        let delta = 3 - count;
        let mut board_clone = board.to_owned();
        for _ in 0..count {
          board_clone.remove(i);
        }
        counter.insert(cur, c - delta);
        let mut temp_c = None;
        while index < board_clone.len() {
          if let Some(ch) = temp_c {
            let mut l = index;
            let mut r = index;
            while l > 0 && board_clone[l - 1] == ch {
              l -= 1;
            }
            while r + 1 < board_clone.len() && board_clone[r + 1] == ch {
              r += 1;
            }
            let temp_count = r - l + 1;
            if temp_count < 3 {
              break;
            } else {
              for j in 0..temp_count {
                board_clone.remove(l);
              }
              temp_c = None;
              index = l;
            }
          } else {
            temp_c = Some(board_clone[index]);
          }
        }
        let res = if board_clone.is_empty() { step + delta } else {
          Solution::find_min_step_dfs(&mut board_clone, 0, counter, step + delta)
        };
        if res != -1 && (ret == -1 || res < ret) {
          ret = res;
        }
        counter.insert(cur, c);
      }
    }
    // 在相同颜色的球间插入的可能，避免出现相同颜色的球消去
    for j in 1..count as usize {
      let options: Vec<char> = counter.iter().filter_map(|(&ch, &cnt)| if ch != cur && cnt > 0 { Some(ch) } else { None }).collect();
      for option in options {
          board.insert(i + j, option);
          *counter.get_mut(&option).unwrap() -= 1;
          let res = Solution::find_min_step_dfs(board, i + j, counter, step + 1);
          if res != -1 && (ret == -1 || res < ret) {
            ret = res;
          }
          *counter.get_mut(&option).unwrap() += 1;
          board.remove(i + j);
      }
    }
    let res = Solution::find_min_step_dfs(board, i + count as usize, counter, step);
    if res != -1 && (ret == -1 || res < ret) {
      ret = res;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    board: &'a str,
    hand: &'a str,
    ret: i32
  }

  #[test]
  fn test_find_min_step_simple() {
    let suites = vec![
      Suite {
        board: "WRRBBW",
        hand: "RB",
        ret: -1
      },
      Suite {
        board: "WWRRBBWW",
        hand: "WRBRW",
        ret: 2
      },
      Suite {
        board: "G",
        hand: "GGGGG",
        ret: 2
      },
      Suite {
        board: "RBYYBBRRB",
        hand: "YRBGB",
        ret: 3
      },
      Suite {
        board: "RRWWRRBBRR",
        hand: "WB",
        ret: 2
      },
      Suite {
        board: "WWBBWBBWW",
        hand: "BB",
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_min_step(s.board.to_owned(), s.hand.to_owned()));
    }
  }
}