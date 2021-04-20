use super::Solution;

impl Solution {
  pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
      return 0;
    }
    if haystack.len() < needle.len() {
      return - 1;
    }
    let chars1: Vec<char> = haystack.chars().collect();
    let chars2: Vec<char> = needle.chars().collect();
    for i in 0..chars1.len() - chars2.len() + 1 {
      for j in 0..chars2.len() {
        if chars1[i + j] != chars2[j] {
          break;
        }
        if j == chars2.len() - 1 {
          return i as i32;
        }
      }
    }
    -1
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    haystack: &'a str,
    needle: &'a str,
    ret: i32
  }

  #[test]
  fn test_str_str_simple() {
    let suites = vec![
      Suite {
        haystack: "hello",
        needle: "ll",
        ret: 2
      },
      Suite {
        haystack: "aaaaa",
        needle: "bba",
        ret: -1
      },
      Suite {
        haystack: "",
        needle: "",
        ret: 0
      },
      Suite {
        haystack: "abb",
        needle: "abaaa",
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(Solution::str_str(s.haystack.to_owned(), s.needle.to_owned()), s.ret);
    }
  }
}