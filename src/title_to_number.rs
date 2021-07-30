use super::Solution;

impl Solution {
  pub fn title_to_number(column_title: String) -> i32 {
    let mut base = 1;
    let mut ret = 0;
    for c in column_title.chars().rev() {
      ret += base * (c as i32 - 64);
      if base < 82595525 {
        base *= 26;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    column_title: &'a str,
    ret: i32
  }

  #[test]
  fn test_title_to_number_simple() {
    let suites = vec![
      Suite {
        column_title: "A",
        ret: 1
      },
      Suite {
        column_title: "AB",
        ret: 28
      },
      Suite {
        column_title: "ZY",
        ret: 701
      },
      Suite {
        column_title: "FXSHRXW",
        ret: 2147483647
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::title_to_number(s.column_title.to_owned()));
    }
  }
}