use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
    arr.sort_unstable();
    let m = 1e9 as i32 + 7;
    let mut dp: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in arr.iter().enumerate() {
      dp.insert(num, 1);
      for j in 0..i {
        if num % arr[j] == 0 {
          let mut delta = 0;
          if let Some(&c1) = dp.get(&(num / arr[j])) {
            if let Some(&c2) = dp.get(&arr[j]) {
              delta = ((c1 as i64 * c2 as i64) % m as i64) as i32;
            }
          }
          if delta != 0 {
            if let Some(c) = dp.get_mut(&num) {
              *c = (*c + delta) % m;
            }
          }
        }
      }
    }
    dp.into_iter().fold(0, |acc, item| (acc + item.1) % m)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    arr: Vec<i32>,
    ret: i32,
  }

  #[test]
  fn test_num_factored_binary_trees_simple() {
    let suites = vec![
      Suite {
        arr: vec![2, 4],
        ret: 3,
      },
      Suite {
        arr: vec![2, 4, 5, 10],
        ret: 7,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::num_factored_binary_trees(s.arr));
    }
  }
}
