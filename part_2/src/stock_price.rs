use std::collections::HashMap;

use solution::Utility;

use crate::Solution;

struct StockPrice {
  time_to_price: HashMap<i32, i32>,
  prices: Vec<i32>,
  current_time: i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {

    fn new() -> Self {
      return StockPrice {
        time_to_price: HashMap::new(),
        prices: Vec::new(),
        current_time: 0
      }
    }
    
    fn update(&mut self, timestamp: i32, price: i32) {
      if !self.time_to_price.contains_key(&timestamp) {
        self.time_to_price.insert(timestamp, price);
      } else {
        let old_price = self.time_to_price.get_mut(&timestamp).unwrap();
        self.prices.remove(Solution::binary_search(&self.prices, *old_price, false) as usize);
        *old_price = price;
      }

      let index = Solution::binary_search(&self.prices, price, true) as usize;
      if index < self.prices.len() {
        self.prices.insert(index, price);
      } else {
        self.prices.push(price);
      }
      
      if timestamp > self.current_time {
        self.current_time = timestamp;
      }
    }
    
    fn current(&self) -> i32 {
      if self.current_time == 0 {
        0
      } else {
        *self.time_to_price.get(&self.current_time).unwrap()
      }
    }
    
    fn maximum(&self) -> i32 {
      if self.current_time == 0 {
        0
      } else {
        *self.prices.last().unwrap()
      }
    }
    
    fn minimum(&self) -> i32 {
      if self.current_time == 0 {
        0
      } else {
        *self.prices.first().unwrap()
      }
    }
}