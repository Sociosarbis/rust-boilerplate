use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
      return false;
    }
    let mut counter1 = [0; 26];
    let mut counter2 = [0; 26];
    for c in word1.bytes() {
      counter1[c as usize - 97] += 1;
    }
    for c in word2.bytes() {
      counter2[c as usize - 97] += 1;
    }
    for i in 0..26 {
      if (counter1[i] != 0 && counter2[i] == 0) || (counter1[i] == 0 && counter2[i] != 0) {
        return false;
      }
    }
    let mut map = HashMap::with_capacity(26);
    for num in counter1 {
      if let Some(c) = map.get_mut(&num) {
        *c += 1;
      } else {
        map.insert(num, 1);
      }
    }
    for num in counter2 {
      if let Some(c) = map.get_mut(&num) {
        *c -= 1;
        if *c < 0 {
          return false;
        }
      } else {
        return false;
      }
    }
    map.values().all(|&c| c == 0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    word1: String,
    word2: String,
    ret: bool,
  }

  #[test]
  fn test_close_strings_simple() {
    let suites = vec![
      Suite {
        word1: "abc".to_string(),
        word2: "bca".to_string(),
        ret: true,
      },
      Suite {
        word1: "a".to_string(),
        word2: "aa".to_string(),
        ret: false,
      },
      Suite {
        word1: "cabbba".to_string(),
        word2: "abbccc".to_string(),
        ret: true,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::close_strings(s.word1, s.word2));
    }
  }
}
