use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut dp: HashMap<i32, i32> = HashMap::new();
    let mut ret = 1;
    for &num in &arr {
      let prev = num - difference;
      let mut next_count = if let Some(&count) = dp.get(&num) {
          count
        } else {
          1
      };
      if let Some(&count) = dp.get(&prev) {
        if count + 1 > next_count {
          next_count = count + 1;
          if next_count > ret {
            ret = next_count;
          }
          dp.insert(num, next_count);
        }
      } else if !dp.contains_key(&num) {
        dp.insert(num, 1);
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    difference: i32,
    ret: i32
  }

  #[test]
  fn test_longest_subsequence_simple() {
    let suites = vec![
      Suite {
        arr: vec![1,2,3,4],
        difference: 1,
        ret: 4
      },
      Suite {
        arr: vec![1,3,5,7],
        difference: 1,
        ret: 1
      },
      Suite {
        arr: vec![1,5,7,8,5,3,4,2,1],
        difference: -2,
        ret: 4
      },
      Suite {
        arr: vec![4,12,10,0,-2,7,-8,9,-9,-12,-12,8,8],
        difference: 0,
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::longest_subsequence(s.arr, s.difference));
    }
  }
}