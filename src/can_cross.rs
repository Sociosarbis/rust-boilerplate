use super::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;

impl Solution {
  pub fn can_cross(stones: Vec<i32>) -> bool {
    let mut dp: HashMap<i32, RefCell<HashSet<i32>>> = HashMap::new();
    for i in 0..stones.len() {
      dp.insert(stones[i], RefCell::new(HashSet::new()));
    }
    dp.get(&stones[0]).unwrap().borrow_mut().insert(1);
    let end = stones[stones.len() - 1];
    for i in 0..stones.len() - 1 {
      let options = dp.get(&stones[i]).unwrap();
      for &option in options.borrow().iter() {
        let target = stones[i] + option;
        if dp.contains_key(&target) {
          let mut next_options = dp.get(&target).unwrap().borrow_mut();
          for j in option - 1..option + 2 {
            if j != 0 && !next_options.contains(&j) {
              next_options.insert(j);
            }
          }
        }
      }
      if !dp.get(&end).unwrap().borrow().is_empty() {
        return true
      }
    }
    false
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    stones: Vec<i32>,
    ret: bool
  }

  #[test]
  fn test_can_cross_simple() {
    let suites = vec![
      Suite {
        stones: vec![0,1,3,5,6,8,12,17],
        ret: true
      },
      Suite {
        stones: vec![0,1,2,3,4,8,9,11],
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(Solution::can_cross(s.stones), s.ret);
    }
  }
}