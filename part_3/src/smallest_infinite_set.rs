use std::{cmp::Reverse, collections::BinaryHeap};

struct SmallestInfiniteSet {
  set: [bool; 1001],
  heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
  fn new() -> Self {
    Self {
      set: [true; 1001],
      heap: (1..=1000).map(|num| Reverse(num)).collect(),
    }
  }

  fn pop_smallest(&mut self) -> i32 {
    if let Some(Reverse(num)) = self.heap.pop() {
      self.set[num as usize] = false;
      num
    } else {
      0
    }
  }

  fn add_back(&mut self, num: i32) {
    if !self.set[num as usize] {
      self.set[num as usize] = true;
      self.heap.push(Reverse(num));
    }
  }
}
