use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
    let mut num_to_index = HashMap::new();
    let mut num_to_count = vec![0;target.len() + 1];
    let mut counts = vec![0];
    for i in 0..target.len() {
      num_to_index.insert(target[i], i + 1);
    }
    let mut ret = 0;
    for num in arr {
      if let Some(&index) = num_to_index.get(&num) {
        let mut idx = Solution::binary_search(&counts, index, true) as usize - 1;
        let new_count = num_to_count[counts[idx]] + 1;
        idx += 1;
        while idx < counts.len() && new_count > num_to_count[counts[idx]] {
          counts.remove(idx);
        }
        if !(idx < counts.len() && counts[idx] == index) {
          if idx < counts.len() {
            counts.insert(idx, index);
          } else {
            counts.push(index);
          }
          num_to_count[index] = new_count;
          if new_count > ret {
            ret = new_count;
          }
        }
      }
    }
    target.len() as i32 - ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    target: Vec<i32>,
    arr: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_min_operations_simple() {
    let suites = vec![
      Suite {
        target: vec![5,1,3],
        arr: vec![9,4,2,3,4],
        ret: 2
      },
      Suite {
        target: vec![6,4,8,1,3,2],
        arr: vec![4,7,6,2,3,8,6,1],
        ret: 3
      },
      Suite {
        target: vec![16,7,20,11,15,13,10,14,6,8],
        arr: vec![11,14,15,7,5,5,6,10,11,6],
        ret: 6
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_operations(s.target, s.arr));
    }
  }
}