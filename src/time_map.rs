use super::Solution;

use std::collections::HashMap;

struct TimeMap {
  map: HashMap<String, (Vec<String>, Vec<i32>)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
      TimeMap {
        map: HashMap::new(),
      }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
      if !self.map.contains_key(&key) {
        self.map.insert(key.to_owned(), (vec![], vec![]));
      }
      if let Some(list_pair) = self.map.get_mut(&key) {
        list_pair.1.push(timestamp);
        list_pair.0.push(value);
      }
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
      if let Some(list_pair) = self.map.get(&key) {
        let index = Solution::binary_search(&list_pair.1, timestamp, true) as usize;
        if index < list_pair.1.len() && list_pair.1[index] == timestamp {
          return list_pair.0[index].to_owned();
        } else if index > 0 {
            return list_pair.0[index - 1].to_owned();
        }
      }
      String::new()
    }
}