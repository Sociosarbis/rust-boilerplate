use super::Solution;

impl Solution {
  pub fn reverse_parentheses(s: String) -> String {
    if s.is_empty() {
      return s;
    }
    let chars: Vec<char> = s.chars().collect();
    let mut stack: Vec<Vec<char>> = vec![];
    for i in 0..chars.len() {
      match chars[i] {
        '(' => {
          stack.push(vec![]);
        },
        ')' => {
          let mut old_top = stack.pop().unwrap();
          Solution::reverse_char_seq(&mut old_top);
          if !stack.is_empty() {
            let new_top = stack.last_mut().unwrap();
            new_top.append(&mut old_top);
          } else {
            stack.push(old_top);
          }
        },
        num => {
          if stack.is_empty() {
            stack.push(vec![]);
          }
          let top = stack.last_mut().unwrap();
          top.push(num);
        }
      }
    }
    stack[0].iter().collect()
  }

  fn reverse_char_seq(chars: &mut Vec<char>) {
    if !chars.is_empty() {
      let mut i = 0;
      let mut j = chars.len() - 1;
      while i < j {
        chars.swap(i, j);
        i += 1;
        j -= 1;
      }
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: &'a str
  }

  #[test]
  fn test_reverse_parentheses_simple() {
    let suites = vec![
      Suite {
        s: "(abcd)",
        ret: "dcba"
      },
      Suite {
        s: "(u(love)i)",
        ret: "iloveu"
      },
      Suite {
        s: "(ed(et(oc))el)",
        ret: "leetcode"
      },
      Suite {
        s: "a(bcdefghijkl(mno)p)q",
        ret: "apmnolkjihgfedcbq"
      }
    ];

    for s in suites {
      assert_eq!(Solution::reverse_parentheses(s.s.to_owned()), s.ret.to_owned());
    }
  }
}