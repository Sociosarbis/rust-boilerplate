use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn largest_vals_from_labels(
    values: Vec<i32>,
    labels: Vec<i32>,
    mut num_wanted: i32,
    use_limit: i32,
  ) -> i32 {
    let mut queue = Vec::with_capacity(values.len());
    for (i, value) in values.into_iter().enumerate() {
      queue.push((value, labels[i]));
    }
    queue.sort_unstable();

    let mut counter = HashMap::new();

    let mut ret = 0;
    
    let mut i = queue.len();

    while num_wanted != 0 && i > 0 {
      let (value, label) = queue[i - 1];
      if let Some(c) = counter.get_mut(&label) {
        if *c < use_limit {
          *c += 1;
        } else {
          i -= 1;
          continue;
        }
      } else {
        counter.insert(label, 1);
      }
      i -= 1;
      ret += value;
      num_wanted -= 1;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    values: Vec<i32>,
    labels: Vec<i32>,
    num_wanted: i32,
    use_limit: i32,
    ret: i32,
  }

  #[test]
  fn test_largest_vals_from_labels_simple() {
    let suites = vec![
      Suite {
        values: vec![5, 4, 3, 2, 1],
        labels: vec![1, 1, 2, 2, 3],
        num_wanted: 3,
        use_limit: 1,
        ret: 9,
      },
      Suite {
        values: vec![5, 4, 3, 2, 1],
        labels: vec![1, 3, 3, 3, 2],
        num_wanted: 3,
        use_limit: 2,
        ret: 12,
      },
      Suite {
        values: vec![9, 8, 8, 7, 6],
        labels: vec![0, 0, 0, 1, 1],
        num_wanted: 3,
        use_limit: 1,
        ret: 16,
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::largest_vals_from_labels(s.values, s.labels, s.num_wanted, s.use_limit)
      );
    }
  }
}
