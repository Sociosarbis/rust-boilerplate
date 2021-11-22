use rand;
use rand::Rng;

struct Solution {
  orig_nums: Vec<i32>,
  mut_nums: Vec<i32>,
  rng: rand::prelude::ThreadRng
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
      return Self {
        orig_nums: nums.clone(),
        mut_nums: nums,
        rng: rand::thread_rng()
      }
    }
    
    fn reset(&mut self) -> Vec<i32> {
      self.mut_nums = self.orig_nums.clone();
      self.mut_nums.clone()
    }
    
    fn shuffle(&mut self) -> Vec<i32> {
      for i in 1..self.mut_nums.len() {
        let j = self.rng.gen_range(0..i+1);
        if j != i {
          let temp = self.mut_nums[j];
          self.mut_nums[j] = self.mut_nums[i];
          self.mut_nums[i] = temp; 
        }
      }
      self.mut_nums.clone()
    }
}