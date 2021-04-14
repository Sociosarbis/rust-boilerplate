use super::*;

use std::collections::HashMap;

struct Trie {
  has_stop: bool,
  children: HashMap<char, Trie>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
      Trie {
        has_stop: false,
        children: HashMap::new()
      }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
      let mut cur = self;
      let chars: Vec<char> = word.chars().collect();
      for i in 0..chars.len() {
        if !cur.children.contains_key(&chars[i]) {
          cur.children.insert(chars[i], Trie::new());
        }
        cur = cur.children.get_mut(&chars[i]).unwrap();
      }
      cur.has_stop = true;
    }
    
    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
      let mut cur = self;
      let chars: Vec<char> = word.chars().collect();
      for i in 0..chars.len() {
        if !cur.children.contains_key(&chars[i]) {
          return false;
        }
        cur = cur.children.get_mut(&chars[i]).unwrap();
      }
      cur.has_stop
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
      let mut cur = self;
      let chars: Vec<char> = prefix.chars().collect();
      for i in 0..chars.len() {
        if !cur.children.contains_key(&chars[i]) {
          return false;
        }
        cur = cur.children.get_mut(&chars[i]).unwrap();
      }
      true
    }
}