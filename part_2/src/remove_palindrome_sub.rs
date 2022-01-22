use super::*;


impl Solution {
  pub fn remove_palindrome_sub(s: String) -> i32 {
    let mut chars = s.chars();
    while let Some(c1) = chars.next() {
      if let Some(c2) = chars.next_back() {
        if c1 != c2 {
          return 2;
        }
      }
    }
    1
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
  fn test_remove_palindrome_sub_simple() {
    let suites = vec![
      Suite {
        s: "ababa",
        ret: 1
      },
      Suite {
        s: "abb",
        ret: 2
      },
      Suite {
        s: "baabb",
        ret: 2
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::remove_palindrome_sub(s.s.to_string()));
    }
  }
}