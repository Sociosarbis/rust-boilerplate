use std::collections::HashMap;

struct Entry {
  has_word: bool,
  next: HashMap<char, Entry>
}

struct WordDictionary (Entry);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
      WordDictionary(Entry{
        has_word: false,
        next: HashMap::new()
      })
    }
    
    fn add_word(&mut self, word: String) {
      let mut cur = &mut self.0;
      let mut i = 0;
      for c in word.chars() {
        if cur.next.get(&c).is_none() {
          cur.next.insert(c, Entry {
            has_word: if i + 1 == word.len() { true } else { false },
            next: HashMap::new()
          });
        }
        cur = cur.next.get_mut(&c).unwrap();
        i += 1;
      }
    }
    
    fn search(&self, word: String) -> bool {
      let chars: Vec<char> = word.chars().collect();
      self.search_dfs(&self.0, &chars, 0)
    }

    fn search_dfs(&self, entry: &Entry, chars: &Vec<char>, index: usize) -> bool {
      if index == chars.len() {
        return entry.has_word;
      }
      match chars[index] {
        '.' => {
          for (_, v) in &entry.next {
            if self.search_dfs(v, chars, index + 1) {
              return true;
            }
          }
        }
        c => {
          if let Some(e) = entry.next.get(&c) {
            if self.search_dfs(e, chars, index + 1) {
              return true;
            }
          }
        }
      }
      false
    }
}