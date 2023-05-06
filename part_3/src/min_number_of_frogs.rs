use super::*;
use std::{collections::HashMap, iter::FromIterator};

impl Solution {
  pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let m: HashMap<u8, usize> = HashMap::from_iter("croak".bytes().enumerate().map(|(i, b)| (b, i)));
    let mut counter = [0;5];
    for c in croak_of_frogs.bytes() {
      let index = *m.get(&c).unwrap();
      if counter[index] == 0 {
        if index != 0 {
          return -1;
        }
        counter[1] += 1;
      } else {
        counter[index] -= 1;
        counter[(index + 1) % 5] += 1;
      }
    }
    if counter.iter().skip(1).any(|&item| item != 0) {
      return -1;
    }
    counter[0]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    croack_of_frogs: String,
    ret: i32
  }

  #[test]
  fn test_min_number_of_frogs_simple() {
    let suites = vec![
      Suite {
        croack_of_frogs: "croakcroak".to_string(),
        ret: 1
      },
      Suite {
        croack_of_frogs: "crcoakroak".to_string(),
        ret: 2
      },
      Suite {
        croack_of_frogs: "croakcrook".to_string(),
        ret: -1
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::min_number_of_frogs(s.croack_of_frogs));
    }
  }
}

