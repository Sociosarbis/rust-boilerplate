use super::*;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
  pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let first_len = first_len as usize;
    let second_len = second_len as usize;
    let mut prefix_sum = vec![0;nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
      prefix_sum[i + 1] = num + prefix_sum[i];
    }
    let n = nums.len();
    let mut left = prefix_sum[first_len];
    let mut rights: BinaryHeap<i32> = BinaryHeap::new();
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for i in first_len..=n - second_len {
      let right = prefix_sum[i + second_len] - prefix_sum[i];
      rights.push(right);
      if let Some(c) = counter.get_mut(&right) {
        *c += 1;
      } else {
        counter.insert(right, 1);
      }
    }
    let mut ret = left + rights.peek().unwrap();
    for i in 1..=n - first_len {
      let index = i + first_len - 1;
      left += nums[index] - nums[i - 1];
      if index + second_len < n + 1 {
        let pop_right = prefix_sum[index + second_len] - prefix_sum[index];
        if let Some(c) = counter.get_mut(&pop_right) {
          *c -= 1;
        }
        if !rights.is_empty() {
          while let Some(c) = counter.get(rights.peek().unwrap()) {
            if *c <= 0 {
              rights.pop();
              if rights.is_empty() {
                break;
              }
            } else {
              break;
            }
          }
        }
      }
      if i >= second_len {
        let right = prefix_sum[i] - prefix_sum[i - second_len];
        rights.push(right);
        if let Some(c) = counter.get_mut(&right) {
          *c += 1;
        } else {
          counter.insert(right, 1);
        }
      }
      if !rights.is_empty() {
        let temp = left + rights.peek().unwrap();
        if temp > ret {
          ret = temp;
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
    nums: Vec<i32>,
    first_len: i32,
    second_len: i32,
    ret: i32
  }

  #[test]
  fn test_max_sum_two_no_overlap_simple() {
    let suites = vec![
      Suite {
        nums: vec![0,6,5,2,2,5,1,9,4],
        first_len: 1,
        second_len: 2,
        ret: 20
      },
      Suite {
        nums: vec![3,8,1,3,2,1,8,9,0],
        first_len: 3,
        second_len: 2,
        ret: 29
      },
      Suite {
        nums: vec![2,1,5,6,0,9,5,0,3,8],
        first_len: 4,
        second_len: 3,
        ret: 31
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_sum_two_no_overlap(s.nums, s.first_len, s.second_len));
    }
  }
}