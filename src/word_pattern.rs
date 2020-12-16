use std::collections::hash_map::HashMap;

struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn word_pattern(pattern: String, s: String) -> bool {
    let right: Vec<&str> = s.split(" ").collect();
    if pattern.len() != right.len() {
      return false;
    }
    let mut records: HashMap<char, &str> = HashMap::new();
    let mut rev_records: HashMap<&str, char> = HashMap::new();
    for (i, c) in pattern.chars().enumerate() {
      let has_left = records.contains_key(&c);
      let has_right = rev_records.contains_key(right[i]);
      if has_left != has_right {
        return false
      }
      if has_left {
        if records.get(&c).unwrap() != &right[i] || rev_records.get(right[i]).unwrap() != &c {
          return false;
        }
      } else {
        records.insert(c, right[i]);
        rev_records.insert(right[i], c);
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_pattern_simple() {
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()), true);
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()), false);
        assert_eq!(Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()), false);
        assert_eq!(Solution::word_pattern("abba".to_owned(), "dog dog dog dog".to_owned()), false);
    }
}