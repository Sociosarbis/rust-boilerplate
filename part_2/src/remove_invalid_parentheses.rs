use super::*;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
  pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut options: HashMap<String, i32> = HashMap::new();
    options.insert(String::new(), 0);
    let mut right = s.chars().fold(0, |acc, item| if item == ')' { acc + 1 } else { acc });
    for c in s.chars() {
      let mut new_options: HashMap<String, i32>;
      match c {
        '(' => {
          new_options = options.clone();
          for opt in options {
            if opt.1 + 1 <= right {
              new_options.insert({
                let mut s = opt.0;
                s.push(c);
                s 
              }, 
              opt.1 + 1);
            }
          }
        }
        ')' => {
          new_options = options.clone();
          right -= 1;
          for opt in options {
              if opt.1 - 1 >= 0 {
                new_options.insert({
                  let mut s = opt.0;
                  s.push(c);
                  s 
                }, 
                opt.1 - 1);
              }
          }
        }
        _ => {
          new_options = HashMap::new();
          for opt in options {
            new_options.insert({
              let mut s = opt.0;
              s.push(c);
              s 
            }, 
            opt.1);
          }
        }
      }
      options = new_options;
    }
    let ret: Vec<String> = options.drain().filter_map(|item| if item.1 == 0 { Some(item.0) } else { None }).collect();
    let max = ret.iter().fold(0, |acc, item| if item.len() > acc { item.len() } else { acc });
    ret.into_iter().filter(|item| item.len() == max).collect()
  }

  pub fn remove_invalid_parentheses_optimal(s: String) -> Vec<String> {
    let (l_remove, r_remove): (i32, i32) = s.chars().fold((0, 0), |acc, item| match item {
      '(' => (acc.0 + 1, acc.1),
      ')' => if acc.0 > 0 { ( acc.0 - 1, acc.1) } else { (acc.0, acc.1 + 1) },
      _ => acc
    });
    let mut valid = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    Solution::remove_invalid_parentheses_dfs(&chars, 0, 0, 0, l_remove, r_remove, &mut String::new(), &mut valid);
    valid.drain().collect()
  }

  pub fn remove_invalid_parentheses_dfs(chars: &Vec<char>, i: usize, l_cnt: i32, r_cnt: i32, l_remove: i32, r_remove: i32, temp: &mut String, valid: &mut HashSet<String>) {
    if i >= chars.len() {
      if l_remove == 0 && r_remove == 0 {
        valid.insert(temp.clone());
      }
      return;
    }
    match chars[i] {
      '(' => {
        if l_remove > 0 {
          Solution::remove_invalid_parentheses_dfs(chars, i + 1, l_cnt, r_cnt, l_remove - 1, r_remove, temp, valid);
        }
        temp.push(chars[i]);
        Solution::remove_invalid_parentheses_dfs(chars, i + 1, l_cnt + 1, r_cnt, l_remove, r_remove, temp, valid);
        temp.pop();
      }
      ')' => {
        if r_remove > 0 {
          Solution::remove_invalid_parentheses_dfs(chars, i + 1, l_cnt, r_cnt, l_remove, r_remove - 1, temp, valid);
        }
        if l_cnt > r_cnt {
          temp.push(chars[i]);
          Solution::remove_invalid_parentheses_dfs(chars, i + 1, l_cnt, r_cnt, l_remove, r_remove, temp, valid);
          temp.pop();
        }
      }
      _ => {
        temp.push(chars[i]);
        Solution::remove_invalid_parentheses_dfs(chars, i + 1, l_cnt, r_cnt, l_remove, r_remove, temp, valid);
        temp.pop();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    ret: Vec<String>
  }

  #[test]
  fn test_remove_invalid_parentheses_simple() {
    let suites = vec![
      Suite {
        s: "()())()",
        ret: t1!["(())()","()()()"]
      },
      Suite {
        s: "(a)())()",
        ret: t1!["(a())()","(a)()()"]
      },
      Suite {
        s: ")(",
        ret: t1![""]
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::remove_invalid_parentheses(s.s.to_owned()));
    }
  }
}