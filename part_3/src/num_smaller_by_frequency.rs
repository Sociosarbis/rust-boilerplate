use super::*;

use std::cmp::Ordering;

impl Solution {
  pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
    let mut ret = vec![0; queries.len()];
    let mut counter = vec![0; words.len()];
    let f = |word: &String| {
      let mut counter = (123, 0);
      for b in word.bytes() {
        match counter.0.cmp(&b) {
          Ordering::Greater => {
            counter.0 = b;
            counter.1 = 1;
          }
          Ordering::Equal => {
            counter.1 += 1;
          }
          _ => {}
        }
      }
      counter.1
    };
    for (i, word) in words.into_iter().enumerate() {
      counter[i] = f(&word);
    }
    counter.sort_unstable();
    for (i, query) in queries.into_iter().enumerate() {
      let count = f(&query);
      let mut l = 0;
      let mut r = counter.len() - 1;
      while l <= r {
        let mid = (l + r) / 2;
        if counter[mid] > count {
          if mid > 0 && counter[mid - 1] > count {
            r = mid - 1;
          } else {
            l = mid;
            break;
          }
        } else {
          l = mid + 1;
        }
      }
      ret[i] = (counter.len() - l) as i32;
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    queries: Vec<String>,
    words: Vec<String>,
    ret: Vec<i32>,
  }

  #[test]
  fn test_num_smaller_by_frequency_simple() {
    let suites = vec![
      Suite {
        queries: t1!["cbd"],
        words: t1!["zaaaz"],
        ret: vec![1],
      },
      Suite {
        queries: t1!["bbb", "cc"],
        words: t1!["a", "aa", "aaa", "aaaa"],
        ret: vec![1, 2],
      },
    ];

    for s in suites {
      assert_eq!(
        s.ret,
        Solution::num_smaller_by_frequency(s.queries, s.words)
      );
    }
  }
}
