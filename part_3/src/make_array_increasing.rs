use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    arr2.sort();
    let mut dp: HashMap<i32, i32> = HashMap::new();
    dp.insert(-1, 0);
    for num in arr1 {
      let mut next_dp: HashMap<i32, i32> = HashMap::new();
      for (k, v) in dp {
        if k < num {
          if let Some(c) = next_dp.get_mut(&num) {
            if *c > v {
              *c = v;
            }
          } else {
            next_dp.insert(num, v);
          }
        }
        let mut l = 0;
        let mut r = arr2.len() - 1;
        while l <= r {
          let mid = (l + r) >> 1;
          if arr2[mid] > k {
            if mid > 0 && arr2[mid - 1] > k {
              r = mid - 1;
            } else {
              l = mid;
              break;
            }
          } else {
            l = mid + 1;
          }
        }
        if l < arr2.len() && (arr2[l] < num || num <= k) {
          if let Some(c) = next_dp.get_mut(&arr2[l]) {
            if *c > v + 1 {
              *c = v + 1;
            }
          } else {
            next_dp.insert(arr2[l], v + 1);
          }
        }
      }
      if next_dp.is_empty() {
        return -1;
      }
      dp = next_dp;
    }
    dp.into_iter().map(|item| item.1).min().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr1: Vec<i32>,
    arr2: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_make_array_increasing_simple() {
    let suites = vec![
      Suite {
        arr1: vec![1,5,3,6,7],
        arr2: vec![1,3,2,4],
        ret: 1
      },
      Suite {
        arr1: vec![1,5,3,6,7],
        arr2: vec![4,3,1],
        ret: 2
      },
      Suite {
        arr1: vec![1,5,3,6,7],
        arr2: vec![1,6,3,3],
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::make_array_increasing(s.arr1, s.arr2));
    }
  }
}