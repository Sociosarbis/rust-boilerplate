use super::*;
use rand;
use rand::prelude::ThreadRng;
use rand::Rng;

struct SolutionPickIndex {
  prefix_sum: Vec<i32>,
  rng: ThreadRng
}

impl SolutionPickIndex {

  fn new(w: Vec<i32>) -> Self {
    let mut prefix_sum = vec![0;w.len() + 1];
    for i in 0..w.len() {
      prefix_sum[i + 1] = prefix_sum[i] + w[i];
    }
    Self {
      prefix_sum,
      rng: rand::thread_rng()
    }
  }
  
  fn pick_index(&mut self) -> i32 {
    let target = self.rng.gen_range(0..*self.prefix_sum.last().unwrap());
    let i = Solution::binary_search(&self.prefix_sum, target, true) as usize;
    if self.prefix_sum[i] == target { i as i32 } else { i as i32 - 1 }
  }
}