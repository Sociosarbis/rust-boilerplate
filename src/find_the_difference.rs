struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn find_the_difference(s: String, t: String) -> char {
      let mut dict = [0;26];
      for c in s.chars() {
        dict[c as usize - 97] += 1;
      }
      for c in t.chars() {
        let i = c as usize - 97;
        dict[i] -= 1;
        if dict[i] < 0 {
          return c;
        }
      }
      'a'
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_the_difference_simple() {
    assert_eq!(Solution::find_the_difference("abcd".to_owned(), "abcde".to_owned()), 'e');
    assert_eq!(Solution::find_the_difference("".to_owned(), "t".to_owned()), 't');
    assert_eq!(Solution::find_the_difference("a".to_owned(), "aa".to_owned()), 'a');
  }
}