use super::*;

impl Solution {
  pub fn balanced_string_split(s: String) -> i32 {
    let mut l = 0;
    let mut r = 0;
    let mut ret = 0;
    for c in s.bytes() {
      match c {
        82 => {
          r += 1;
        }
        76 => {
          l += 1;
        }
        _ => {}
      }
      if l == r {
        ret += 1;
        l = 0;
        r = 0;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: i32
  }

  #[test]
  fn test_balenced_string_split_simple() {
    let suites = vec![
      Suite {
        s: "RLRRLLRLRL",
        ret: 4
      },
      Suite {
        s: "RLLLLRRRLR",
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::balanced_string_split(s.s.to_owned()));
    }
  }
}