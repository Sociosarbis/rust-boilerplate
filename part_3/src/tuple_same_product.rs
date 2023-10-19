use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut counter = HashMap::new();
    for (i, a) in nums.iter().enumerate() {
      for j in i + 1..nums.len() {
        let temp = a * nums[j];
        if let Some(c) = counter.get_mut(&temp) {
          *c += 1;
        } else {
          counter.insert(temp, 1);
        }
      }
    }
    counter.into_iter().fold(0, |acc, (_, count)| {
      acc
        + if count == 1 {
          0
        } else {
          (count - 1) * count * 4
        }
    })
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
  fn test_tuple_same_product_simple() {
    let suites = vec![
      Suite {
        nums: vec![2, 3, 4, 6],
        ret: 8,
      },
      Suite {
        nums: vec![1, 2, 4, 5, 10],
        ret: 16,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::tuple_same_product(s.nums));
    }
  }
}
