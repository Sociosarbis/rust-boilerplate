use super::*;

use std::cmp::min;

impl Solution {
  pub fn repeated_string_match(a: String, b: String) -> i32 {
    let mut c_a: Vec<char> = a.chars().collect();
    let c_b: Vec<char> = b.chars().collect();
    let mut repeated_len = 1;
    for i in 1..c_b.len() {
      if c_b[i] == c_b[i - 1] {
        repeated_len += 1;
      } else {
        break;
      }
    }
    let n = a.len();
    let mut start = 0;
    for i in 0..n {
      let mut found = true;
      for j in start..c_b.len() {
        while i + j >= c_a.len() {
          c_a.extend(a.chars());
        }
        if c_a[i + j] != c_b[j] {
          found = false;
          start = if j == 0 { 0 } else { min(j, repeated_len) - 1 };
          break;
        }
      }
      if found {
        let l = i + c_b.len();
        return (l / a.len() + if l % a.len() == 0 { 0 } else { 1 }) as i32;
      }
    }
    -1
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite <'a> {
    a: &'a str,
    b: &'a str,
    ret: i32
  }

  #[test]
  fn test_repeated_string_match_simple() {
    let suites = vec![
      Suite {
        a: "abcd",
        b: "cdabcdab",
        ret: 3,
      },
      Suite {
        a: "a",
        b: "aa",
        ret: 2
      },
      Suite {
        a: "a",
        b: "a",
        ret: 1
      },
      Suite {
        a: "abc",
        b: "wxyz",
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::repeated_string_match(s.a.to_owned(), s.b.to_owned()));
    }
  }
}