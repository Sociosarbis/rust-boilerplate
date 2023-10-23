use super::*;

use std::iter::FromIterator;

impl Solution {
  pub fn count_seniors(details: Vec<String>) -> i32 {
    details.into_iter().fold(0, |acc, d| {
      acc
        + if String::from_iter(d.chars().skip(11).take(2))
          .parse::<i32>()
          .unwrap()
          > 60
        {
          1
        } else {
          0
        }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    details: Vec<String>,
    ret: i32,
  }

  #[test]
  fn test_count_seniors_simple() {
    let suites = vec![
      Suite {
        details: t1!["7868190130M7522", "5303914400F9211", "9273338290F4010"],
        ret: 2,
      },
      Suite {
        details: t1!["1313579440F2036", "2921522980M5644"],
        ret: 0,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::count_seniors(s.details));
    }
  }
}
