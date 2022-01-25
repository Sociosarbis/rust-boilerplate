use super::*;

impl Solution {
  pub fn number_of_matches(n: i32) -> i32 {
    let mut teams = n;
    let mut ret = 0;
    while teams != 1 {
      let matches = teams >> 1;
      ret += matches;
      teams -= matches;
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    n: i32,
    ret: i32
  }

  #[test]
  fn test_number_of_matches_simple() {
    let suites = vec![
      Suite {
        n: 7,
        ret: 6
      },
      Suite {
        n: 14,
        ret: 13
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::number_of_matches(s.n));
    }
  }
}