use super::*;

impl Solution {
  pub fn odd_string(words: Vec<String>) -> String {
    let word1: Vec<u8> = words[0].bytes().collect();
    let mut ret = String::new();
    for word in words.iter().skip(1) {
      let mut it = word.bytes().enumerate();
      let mut prev_b: u8 = 0;
      if let Some((_, b)) = it.next() {
        prev_b = b;
      }
      let mut has_same = false;
      for (i, b) in it {
        // 如果不同
        if b as i8 - prev_b as i8 != word1[i] as i8 - word1[i - 1] as i8 {
          // 如果已存在不同
          if !ret.is_empty() {
            return words[0].clone();
          } else {
            // 如果存在相同
            if has_same {
              return word.clone();
            }
            ret = word.clone();
            break;
          }
          // 如果相同
        } else if !ret.is_empty() {
          return ret;
        } else if !has_same {
          has_same = true;
        }
        prev_b = b;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::*;

  struct Suite {
    words: Vec<String>,
    ret: String,
  }

  #[test]
  fn test_odd_string_simple() {
    let suites = vec![
      Suite {
        words: t1!["adc", "wzy", "abc"],
        ret: "abc".to_string(),
      },
      Suite {
        words: t1!["aaa", "bob", "ccc", "ddd"],
        ret: "bob".to_string(),
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::odd_string(s.words));
    }
  }
}
