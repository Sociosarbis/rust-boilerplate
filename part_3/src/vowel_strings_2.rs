use super::*;

impl Solution {
  pub fn vowel_strings_2(words: Vec<String>, left: i32, right: i32) -> i32 {
    let is_vowel = |b: u8| b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u';
    words
      .into_iter()
      .skip(left as usize)
      .take((right - left + 1) as usize)
      .fold(0, |acc, word| {
        let mut bytes = word.bytes();
        if let Some(b) = bytes.next() {
          if is_vowel(b) {
            if let Some(b) = bytes.next_back() {
              if is_vowel(b) {
                return acc + 1;
              }
            } else {
              return acc + 1;
            }
          }
        }
        acc
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    left: i32,
    right: i32,
    ret: i32,
  }

  #[test]
  fn test_vowel_strings_2_simple() {
    let suites = vec![
      Suite {
        words: t1!["are", "amy", "u"],
        left: 0,
        right: 2,
        ret: 2,
      },
      Suite {
        words: t1!["hey", "aeo", "mu", "ooo", "artro"],
        left: 1,
        right: 4,
        ret: 3,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::vowel_strings_2(s.words, s.left, s.right));
    }
  }
}
