use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn count_triplets(nums: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for &a in &nums {
      for &b in &nums {
        let c = a & b;
        if let Some(d) = counter.get_mut(&c) {
          *d += 1;
        } else {
          counter.insert(c, 1);
        }
      }
    }
    let mut ret = 0;
    for &a in &nums {
      for (k, &v) in &counter {
        if a & k == 0 {
          ret += v;
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
    ret: i32,
  }

  #[test]
  fn test_count_triplets_simple() {
    let suites = vec![
      Suite {
        nums: vec![2,1,3],
        ret: 12
      },
      Suite {
        nums: vec![0,0,0],
        ret: 27
      },
      Suite {
        nums: vec![2,4,7,3],
        ret: 30
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_triplets(s.nums));
    }
  }
}