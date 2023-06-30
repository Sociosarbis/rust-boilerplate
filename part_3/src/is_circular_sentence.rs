use super::*;

impl Solution {
  pub fn is_circular_sentence(sentence: String) -> bool {
    let mut previous_char = sentence.bytes().rev().next().unwrap();
    let mut is_first = true;
    for c in sentence.bytes() {
      if c == b' ' {
        is_first = true;
      } else {
        if is_first {
          if c != previous_char {
            return false;
          }
          is_first = false;
        }
        previous_char = c;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    sentence: String,
    ret: bool,
  }

  #[test]
  fn test_is_circular_sentence_simple() {
    let suites = vec![
      Suite {
        sentence: "leetcode exercises sound delightful".to_string(),
        ret: true,
      },
      Suite {
        sentence: "eetcode".to_string(),
        ret: true,
      },
      Suite {
        sentence: "Leetcode is cool".to_string(),
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::is_circular_sentence(s.sentence));
    }
  }
}
