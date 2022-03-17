use std::{cell::RefCell, rc::Rc};
use std::cmp::Ordering;

use super::*;

#[derive(Clone)]
struct Trie {
    next: Vec<Option<Rc<RefCell<Trie>>>>,
}

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_unstable_by(|a, b| {
          match a.len().cmp(&b.len()) {
            Ordering::Equal => {
              let mut b_iter = b.bytes();
              for c in a.bytes() {
                match c.cmp(&b_iter.next().unwrap()) {
                  Ordering::Equal => {},
                  order => {
                    return order
                  }
                }
              }
              Ordering::Equal
            },
            order => order
          }
        });
        let root = Rc::new(RefCell::new(Trie {
            next: vec![None; 26],
        }));
        let mut l = 0;
        let mut ret = String::new();
        for word in &words {
            if word.len() > l + 1 {
                break;
            } else if l != word.len() {
              l = word.len()
            }
            let mut cur = root.clone();
            let mut is_valid = true;
            for c in word.bytes().take(word.len() - 1) {
                if let Some(res) = cur.clone().borrow().next[c as usize - 97].clone() {
                    cur = res;
                } else {
                  is_valid = false;
                  break;
                }
            }
            if is_valid {
              cur.borrow_mut().next
              [word.bytes().skip(word.len() - 1).next().unwrap() as usize - 97] =
              Some(Rc::new(RefCell::new(Trie {
                  next: vec![None; 26],
              })));
              if ret.len() < word.len() {
                ret = word.clone();
              }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Suite {
        words: Vec<String>,
        ret: String,
    }

    #[test]
    fn test_longest_word_simple() {
        let suites = vec![
            Suite {
                words: t1!["w", "wo", "wor", "worl", "world"],
                ret: "world".to_string(),
            },
            Suite {
                words: t1!["a", "banana", "app", "appl", "ap", "apply", "apple"],
                ret: "apple".to_string(),
            },
        ];

        for s in suites {
            assert_eq!(s.ret, Solution::longest_word(s.words));
        }
    }
}
