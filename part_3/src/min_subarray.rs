use std::collections::HashMap;

use super::*;

impl Solution {
  pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut remain = 0;
    for &num in &nums {
      remain = (remain + num) % p;
    }
    if remain == 0 {
      return 0;
    }
    let mut temp = 0;
    let mut ret = -1;
    let mut m: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
      temp = (temp + num) % p;
      let target = if temp >= remain { temp - remain } else { temp + p - remain };
      if let Some(j) = m.get(&target) {
        let v = (i - j) as i32;
        if ret == -1 || v < ret {
          ret = v;
        }
      } else if target == 0 && i + 1 != nums.len() {
        let v = i as i32 + 1;
        if ret == -1 || v < ret {
          ret = v;
        }
      }
      m.insert(temp, i);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    nums: Vec<i32>,
    p: i32,
    ret: i32
  }

  #[test]
  fn test_min_subarray_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,1,4,2],
        p: 6,
        ret: 1
      },
      Suite {
        nums: vec![6,3,5,2],
        p: 9,
        ret: 2
      },
      Suite {
        nums: vec![1,2,3],
        p: 7,
        ret: -1
      },
      Suite {
        nums: vec![1,1,1],
        p: 2,
        ret: 1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_subarray(s.nums, s.p));
    }
  }
}