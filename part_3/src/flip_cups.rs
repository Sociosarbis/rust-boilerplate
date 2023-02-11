use std::mem::swap;

use super::*;

impl Solution {
  pub fn fill_cups(mut amount: Vec<i32>) -> i32 {
    amount.sort();
    let mut ret = 0;
    while amount[1] != 0 {
      amount[1] -= 1;
      if amount[1] < amount[0] {
        amount.swap(0, 1);
      }
      amount[2] -= 1;
      if amount[2] < amount[1] {
        amount.swap(1, 2);
      }
      ret += 1;
    }
    ret + amount[2]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    amount: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_fill_cups_simple() {
    let suites = vec![
      Suite {
        amount: vec![1,4,2],
        ret: 4,
      },
      Suite {
        amount: vec![5,4,4],
        ret: 7,
      },
      Suite {
        amount: vec![5,0,0],
        ret: 5,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::fill_cups(s.amount));
    }
  }
}