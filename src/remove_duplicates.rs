use super::Solution;

impl Solution {
  pub fn remove_duplicates(s: String) -> String {
    let mut stack: Vec<char> = vec![];
    let chars: Vec<char> = s.chars().collect();
    stack.push(chars[0]);
    for i in 1..chars.len() {
      if chars[i] != stack[stack.len() - 1] {
        let mut j = stack.len() - 1;
        while j >= 1 {
          if stack[j] != stack[j - 1] {
            break;
          } else {
            stack.resize(j - 1, '\0');
          }
          if j < 2 {
            break;
          } else {
            j -= 2;
          }
        }
      }
      stack.push(chars[i]);
    }
    let mut j = stack.len() - 1;
    while j >= 1 {
      if stack[j] != stack[j - 1] {
        break;
      } else {
        stack.resize(j - 1, '\0');
      }
      if j < 2 {
        break;
      } else {
        j -= 2;
      }
    }
    stack.iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s: String,
    ret: String,
  }

  #[test]
  fn remove_duplicates_simple() {
    let suites = vec![
      Suite {
        s: "abbaca".to_string(),
        ret: "ca".to_string()
      },
      Suite {
        s: "aaaaaaaaa".to_string(),
        ret: "a".to_string()
      }
    ];

    for s in suites {
      assert_eq!(Solution::remove_duplicates(s.s), s.ret);
    }
  }
}