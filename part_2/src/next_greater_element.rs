use super::*;
use std::collections::HashMap;

impl Solution {
  pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let map = (0..nums1.len()).fold(HashMap::new(), |mut acc, i| {
      acc.insert(nums1[i], i);
      return acc;
    });
    let mut ret = vec![-1;nums1.len()];
    let mut stack = vec![];
    for &num in &nums2 {
      while !stack.is_empty() {
        if let Some(&cur) = stack.last() {
          if num > cur {
            if let Some(&i) = map.get(&cur) {
              ret[i] = num;
            }
            stack.pop();
          } else {
            break;
          }
        }
      }
      if map.contains_key(&num) {
        stack.push(num);
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    ret: Vec<i32>
  }

  #[test]
  fn test_next_greater_element_simple() {
    let suites = vec![
      Suite {
        nums1: vec![4,1,2],
        nums2: vec![1,3,4,2],
        ret: vec![-1,3,-1]
      },
      Suite {
        nums1: vec![2,4],
        nums2: vec![1,2,3,4],
        ret: vec![3,-1]
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::next_greater_element(s.nums1, s.nums2));
    }
  }
}