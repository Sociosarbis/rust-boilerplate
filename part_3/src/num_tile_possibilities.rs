use super::*;

use std::collections::{HashSet, HashMap};

impl Solution {
  pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut m = HashSet::new();
    let mut counter = HashMap::new();
    let mut chars = vec![];
    for ch in tiles.chars() {
      if let Some(c) = counter.get_mut(&ch) {
        *c += 1;
      } else {
        counter.insert(ch, 1);
        chars.push(ch);
      }
    }
    Solution::num_tile_possibilities_dfs(&mut m, &mut counter, &chars, &mut String::new());
    m.len() as i32
  }

  pub fn num_tile_possibilities_dfs(
    m: &mut HashSet<String>,
    counter: &mut HashMap<char, i32>,
    chars: &Vec<char>,
    temp: &mut String,
  ) {
    for &k in chars {
      if let Some(v) = counter.get_mut(&k) {
        if *v != 0 {
          *v -= 1;
          temp.push(k);
          if !m.contains(temp) {
            m.insert(temp.clone());
          }
        } else {
          continue;
        }
      }
      Solution::num_tile_possibilities_dfs(m, counter, chars, temp);
      if let Some(v) = counter.get_mut(&k) {
        *v += 1;
        temp.pop();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    tiles: String,
    ret: i32,
  }

  #[test]
  fn test_num_tile_possibilities_simple() {
    let suites = vec![
      Suite {
        tiles: "AAB".to_string(),
        ret: 8,
      },
      Suite {
        tiles: "AAABBC".to_string(),
        ret: 188,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_tile_possibilities(s.tiles));
    }
  }
}
