use super::*;
use std::collections::HashMap;

impl Solution {
  pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    let mut max = 0;
    let mut counter = HashMap::new();
    for num in nums {
      if num & 1 == 0 {
        if let Some(count) = counter.get_mut(&num) {
          *count += 1;
          if *count > max {
            ret = num;
            max = *count;
          } else if *count == max {
            if num < ret {
              ret = num;
            }
          } 
        } else {
          counter.insert(num, 1);
          if max == 1 {
            if num < ret {
              ret = num;
            }
          } else if max == 0 {
            ret = num;
            max = 1;
          }
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
    ret: i32
  }

  #[test]
  fn test_most_frequent_even_simple() {
    let suites = vec![
      Suite {
        nums: vec![0,1,2,2,4,4,1],
        ret: 2,
      },
      Suite {
        nums: vec![4,4,4,9,2,4],
        ret: 4,
      },
      Suite {
        nums: vec![29,47,21,41,13,37,25,7],
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::most_frequent_even(s.nums));
    }
  }
}