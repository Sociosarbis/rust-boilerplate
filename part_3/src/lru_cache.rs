use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap},
};

struct LRUCache {
  v: i32,
  capacity: usize,
  heap: BinaryHeap<(Reverse<i32>, i32)>,
  map: HashMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
  fn new(capacity: i32) -> Self {
    Self {
      v: 0,
      capacity: capacity as usize,
      heap: BinaryHeap::new(),
      map: HashMap::new(),
    }
  }

  fn get(&mut self, key: i32) -> i32 {
    if let Some((v, c)) = self.map.get_mut(&key) {
      *c = self.v;
      self.v += 1;
      *v
    } else {
      -1
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    if let Some((v, c)) = self.map.get_mut(&key) {
      *v = value;
      *c = self.v;
    } else {
      if self.map.len() == self.capacity {
        while let Some((Reverse(c1), v)) = self.heap.pop() {
          if let Some(&(_, c2)) = self.map.get(&v) {
            if c1 != c2 {
              self.heap.push((Reverse(c2), v));
            } else {
              self.map.remove(&v);
              break;
            }
          }
        }
      }
      self.map.insert(key, (value, self.v));
      self.heap.push((Reverse(self.v), key));
    }
    self.v += 1;
  }
}
