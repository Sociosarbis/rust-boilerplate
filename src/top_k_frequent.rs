use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
      let mut hash: HashMap<String, i32> = HashMap::new();
      let mut ret = vec![];
      for word in &words {
        if !hash.contains_key(word) {
          hash.insert(word.to_owned(), 0);
        }
        if let Some(count) = hash.get_mut(word) {
          *count += 1;
        }
      }
      for (word, &count) in &hash {
        if ret.is_empty() {
          ret.push(word.to_owned());
        } else {
          let mut l = 0;
          let mut r = ret.len() - 1;
          let mut index = 0;
          while l <= r {
            let mid = (l + r) / 2;
            let mid_val = *hash.get(&ret[mid]).unwrap();
            if mid_val > count || (mid_val == count && Solution::is_alpha_bigger(word, &ret[mid])) {
              l = mid + 1;
              if l > r {
                index = l;
                break;
              }
            } else if mid_val < count || (mid_val == count && !Solution::is_alpha_bigger(word, &ret[mid])){
              if mid == 0 || mid - 1 < l {
                index = mid;
                break;
              }
              r = mid - 1;
            } else {
              index = mid;
              break;
            }
          }
          if index == ret.len() && ret.len() < k as usize {
            ret.push(word.to_owned());
          } else {
            ret.insert(index, word.to_owned());
          }

          if ret.len() > k as usize {
            ret.pop();
          }
        }
      }
      ret
  }

  fn is_alpha_bigger(word_1: &String, word_2: &String) -> bool {
    let mut chars_1 = word_1.chars();
    let mut chars_2 = word_2.chars();
    let mut char_1_opt = chars_1.next();
    let mut char_2_opt = chars_2.next();
    while char_1_opt.is_some() && char_2_opt.is_some() {
      let char_1 = char_1_opt.unwrap();
      let char_2 = char_2_opt.unwrap();
      if char_1 > char_2 {
        return true;
      } else if char_1 < char_2 {
        return false;
      }
      char_1_opt = chars_1.next();
      char_2_opt = chars_2.next();
    }
    word_1.len() > word_2.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    k: i32,
    ret: Vec<String>
  }

  #[test]
  fn test_top_k_frequent_simple() {
    let suites = vec![
      Suite {
        words: t1!["i", "love", "leetcode", "i", "love", "coding"],
        k: 2,
        ret: t1!["i", "love"]
      },
      Suite {
        words: t1!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"],
        k: 4,
        ret: t1!["the", "is", "sunny", "day"]
      },
      Suite {
        words: t1!["plpaboutit","jnoqzdute","sfvkdqf","mjc","nkpllqzjzp","foqqenbey","ssnanizsav","nkpllqzjzp","sfvkdqf","isnjmy","pnqsz","hhqpvvt","fvvdtpnzx","jkqonvenhx","cyxwlef","hhqpvvt","fvvdtpnzx","plpaboutit","sfvkdqf","mjc","fvvdtpnzx","bwumsj","foqqenbey","isnjmy","nkpllqzjzp","hhqpvvt","foqqenbey","fvvdtpnzx","bwumsj","hhqpvvt","fvvdtpnzx","jkqonvenhx","jnoqzdute","foqqenbey","jnoqzdute","foqqenbey","hhqpvvt","ssnanizsav","mjc","foqqenbey","bwumsj","ssnanizsav","fvvdtpnzx","nkpllqzjzp","jkqonvenhx","hhqpvvt","mjc","isnjmy","bwumsj","pnqsz","hhqpvvt","nkpllqzjzp","jnoqzdute","pnqsz","nkpllqzjzp","jnoqzdute","foqqenbey","nkpllqzjzp","hhqpvvt","fvvdtpnzx","plpaboutit","jnoqzdute","sfvkdqf","fvvdtpnzx","jkqonvenhx","jnoqzdute","nkpllqzjzp","jnoqzdute","fvvdtpnzx","jkqonvenhx","hhqpvvt","isnjmy","jkqonvenhx","ssnanizsav","jnoqzdute","jkqonvenhx","fvvdtpnzx","hhqpvvt","bwumsj","nkpllqzjzp","bwumsj","jkqonvenhx","jnoqzdute","pnqsz","foqqenbey","sfvkdqf","sfvkdqf"],
        k: 1,
        ret: t1!["fvvdtpnzx"]
      }
    ];
    for s in suites {
      assert_eq!(Solution::top_k_frequent(s.words, s.k), s.ret);
    }
  }
}