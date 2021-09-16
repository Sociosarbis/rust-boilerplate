use super::*;

use std::collections::HashMap;


struct Entry {
  count: i32,
  has_leaf: bool,
  next: HashMap<char, Entry>
}

impl Solution {
  pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut dict = HashMap::new();
    let mut ret = vec![];
    for word in words {
      let mut cur = &mut dict;
      let chars: Vec<char> = word.chars().collect();
      for i in 0..chars.len() {
        let c = chars[i];
        if !cur.contains_key(&c) {
          cur.insert(c, Entry{
            count: 0,
            has_leaf: false,
            next: HashMap::new()
          });
        }
        if i == chars.len() - 1 {
          cur.get_mut(&c).unwrap().has_leaf = true;
        }
        cur.get_mut(&c).unwrap().count += 1;
        cur = &mut cur.get_mut(&c).unwrap().next;
      }
    }
    let mut temp = String::new();
    for i in 0..board.len() {
      for j in 0..board[i].len() {
        Solution::find_words_dfs(&mut board, i as i32, j as i32, &mut temp, &mut dict, &mut ret);
      }
    }
    ret
  }

  fn find_words_dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, temp: &mut String, dict: &mut HashMap<char, Entry>, ret: &mut Vec<String>) {
    let c = board[i as usize][j as usize];
    let old_size = ret.len();
    if !dict.contains_key(&c) {
      return;
    }
    temp.push(c);
    board[i as usize][j as usize] = '\0';
    let value = dict.get_mut(&c).unwrap();
    let next_dict = &mut value.next;
    if value.has_leaf {
      value.has_leaf = false;
      ret.push(temp.clone());
    }
    let dirs = [(-1,0),(0,1),(1,0),(0,-1)];
    for dir in &dirs {
      let next_i = i + dir.0;
      let next_j = j + dir.1;
      if next_i >= 0 && next_i < board.len() as i32 && next_j >= 0 && next_j < board[next_i as usize].len() as i32 && board[next_i as usize][next_j as usize] != '\0' {
        Solution::find_words_dfs(board, next_i, next_j, temp, next_dict, ret);
      }
    }
    value.count -= (ret.len() - old_size) as i32;
    board[i as usize][j as usize] = c;
    if value.count == 0 {
      dict.remove(&c);
    }
    temp.pop();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    board: Vec<Vec<char>>,
    words: Vec<String>,
    ret: Vec<String>
  }

  #[test]
  fn test_find_words_simple() {
    let suites = vec![
      Suite {
        board: t2_c![["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]],
        words: t1!["oath","pea","eat","rain"],
        ret: t1!["oath", "eat"]
      },
      Suite {
        board: t2_c![["a","b"],["c","d"]],
        words: t1!["abcb"],
        ret: t1![]
      },
      Suite {
        board: t2_c![["o","a","b","n"],["o","t","a","e"],["a","h","k","r"],["a","f","l","v"]],
        words: t1!["oa","oaa"],
        ret: t1!["oa","oaa"]
      },
      Suite {
        board: t2_c![["a","b"],["a","a"]],
        words: t1!["aba","baa","bab","aaab","aaa","aaaa","aaba"],
        ret: t1!["aba","aaa","aaab","baa","aaba"]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_words(s.board, s.words));
    }
  }
}