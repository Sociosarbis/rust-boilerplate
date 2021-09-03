use super::*;
use std::collections::binary_heap::BinaryHeap;

impl Solution {
  pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for num in arr {
      heap.push(num);
      if heap.len() as i32 > k {
        heap.pop();
      }
    }
    heap.into_iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    k: i32,
    ret: Vec<i32>
  }

  #[test]
  fn test_smallest_k_simple() {
    let suites = vec![
      Suite {
        arr: vec![1,3,5,7,2,4,6,8],
        k: 4,
        ret: vec![4,2,3,1]
      }
    ];
    
    for s in suites {
      assert_eq!(s.ret, Solution::smallest_k(s.arr, s.k));
    }
  }
}