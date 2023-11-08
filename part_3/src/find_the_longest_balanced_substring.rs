use super::*;

impl Solution {
  pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let mut ret = 0;
    let mut ones = 0;
    let mut zeroes = 0;
    for b in s.bytes() {
      if b == b'0' {
        if ones != 0 {
          ret = ret.max(ones * 2);
          zeroes = 0;
          ones = 0;
        }
        zeroes += 1;
      } else {
        if ones < zeroes {
          ones += 1;
        }
      }
    }
    if ones != 0 {
      ret = ret.max(ones * 2);
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: i32,
  }

  #[test]
  fn test_find_the_longest_balanced_substring_simple() {
    let suites = vec![
      Suite {
        s: "01000111".to_string(),
        ret: 6,
      },
      Suite {
        s: "00111".to_string(),
        ret: 4,
      },
      Suite {
        s: "111".to_string(),
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_the_longest_balanced_substring(s.s));
    }
  }
}
