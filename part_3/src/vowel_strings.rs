use super::*;

impl Solution {
  pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut prefix_sum = vec![0; words.len() + 1];
    let is_vowel = |b: u8| b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u';
    for (i, word) in words.into_iter().enumerate() {
      prefix_sum[i + 1] = prefix_sum[i];
      if let Some(b) = word.bytes().next() {
        if is_vowel(b) {
          if let Some(b) = word.bytes().next_back() {
            if is_vowel(b) {
              prefix_sum[i + 1] += 1;
            }
          }
        }
      }
    }
    let mut ret = vec![0; queries.len()];
    for (i, q) in queries.into_iter().enumerate() {
      ret[i] = prefix_sum[q[1] as usize + 1] - prefix_sum[q[0] as usize];
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    queries: Vec<Vec<i32>>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_vowel_strings_simple() {
    let suites = vec![
      Suite {
        words: t1!["aba", "bcb", "ece", "aa", "e"],
        queries: t2_i![[0, 2], [1, 4], [1, 1]],
        ret: vec![2, 3, 0],
      },
      Suite {
        words: t1!["a", "e", "i"],
        queries: t2_i![[0, 2], [0, 1], [2, 2]],
        ret: vec![3, 2, 1],
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::vowel_strings(s.words, s.queries));
    }
  }
}
