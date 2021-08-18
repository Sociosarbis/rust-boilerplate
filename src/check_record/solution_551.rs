use crate::Solution;

impl Solution {
  pub fn check_record(s: String) -> bool {
    let mut a_count = 0;
    let mut l_count = 0;
    for c in s.chars() {
      match c {
        'L' => {
          l_count += 1;
          if l_count >= 3 {
            return false;
          }
        }
        'A' => {
          a_count += 1;
          if a_count >= 2 {
            return false;
          }
          l_count = 0;
        }
        _ => {
          l_count = 0;
        }
      }
    }
    true
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: bool
  }

  #[test]
  fn test_check_records_simple() {
    let suites = vec![
      Suite {
        s: "PPALLP",
        ret: true
      },
      Suite {
        s: "PPALLL",
        ret: false
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_record(s.s.to_owned()));
    }
  }
}