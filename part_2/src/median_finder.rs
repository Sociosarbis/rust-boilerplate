use super::*;

use std::collections::binary_heap::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
  arr: Vec<i32>
}

impl Utility for MedianFinder {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
      return MedianFinder {
        arr: vec![]
      };
    }
    
    fn add_num(&mut self, num: i32) {
      let index = Self::binary_search(&self.arr, num, true) as usize;
      if index < self.arr.len() {
        self.arr.insert(index, num);
      } else {
        self.arr.push(num);
      }
    }
    
    fn find_median(&self) -> f64 {
      if self.arr.len() & 1 == 0 {
        self.arr[self.arr.len() >> 1] as f64
      } else {
        (self.arr[self.arr.len() >> 1] as f64 + self.arr[(self.arr.len() >> 1) - 1] as f64) / 2.0
      }
    }
}


struct MedianFinderBest {
  low: BinaryHeap<i32>,
  high: BinaryHeap<Reverse<i32>>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinderBest {

    /** initialize your data structure here. */
    fn new() -> Self {
      return MedianFinderBest {
        low: BinaryHeap::new(),
        high: BinaryHeap::new()
      };
    }
    
    fn add_num(&mut self, num: i32) {
      self.low.push(num);
      self.high.push(Reverse(self.low.pop().unwrap()));
      if self.high.len() > self.low.len() {
        self.low.push(self.high.pop().unwrap().0);
      }
    }
    
    fn find_median(&self) -> f64 {
      let l = *self.low.peek().unwrap() as f64;
      if self.low.len() == self.high.len() {
        return (l + self.high.peek().unwrap().0 as f64) / 2.0;
      }
      l
    }
}
