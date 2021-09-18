use super::*;

impl Solution {
  pub fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: bool
  }

  #[test]
  fn test_can_win_nim_simple() {
    let suites = vec![
      Suite {
        4,
        false
      },
      Suite {
        1,
        true
      },
      Suite {
        2,
        true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::can_win_nim(s.n));
    }
  }
}