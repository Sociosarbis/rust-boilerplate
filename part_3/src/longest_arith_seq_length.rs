use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut dp: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut ret = 2;
    for (i, &num) in nums.iter().enumerate() {
      for j in 0..=i {
        let diff = num - nums[j];
        if diff == 0 && j != i {
          continue;
        }
        if let Some(m1) = dp.get_mut(&diff) {
          if let Some(m2) = m1.get(&nums[j]) {
            let v = *m2 + 1;
            if v > ret {
              ret = v;
            }
            m1.insert(num, v);
          } else {
            m1.insert(num, if j == i { 1 } else {  2  });
          }
        } else {
          let mut m1: HashMap<i32, i32> = HashMap::new();
          m1.insert(num, if j == i { 1 } else {  2  });
          dp.insert(diff, m1);
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
  fn test_longest_arith_seq_length_simple() {
    let suites = vec![
      Suite {
        nums: vec![3,6,9,12],
        ret: 4,
      },
      Suite {
        nums: vec![9,4,7,2,10],
        ret: 3,
      },
      Suite {
        nums: vec![20,1,15,3,10,5,8],
        ret: 4,
      },
      Suite {
        nums: vec![24,13,1,100,0,94,3,0,3],
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::longest_arith_seq_length(s.nums));
    }
  }
}