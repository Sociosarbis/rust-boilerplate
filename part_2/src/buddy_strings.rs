use super::*;

impl Solution {
  pub fn buddy_strings(s: String, goal: String) -> bool {
      if s.len() != goal.len() {
          return false;
      }
      let mut counter = vec![0;26];
      let mut s_iter = s.bytes();
      let mut goal_iter = goal.bytes();
      let mut left = None;
      let mut right = None;
      let mut mask = 0;
      while let Some(c1) = s_iter.next() {
        let c2 = goal_iter.next().unwrap();
        if c1 != c2 {
          if let Some(l) = left {
            let r = right.unwrap();
            if (mask & 1) != 0 || l != c2 || c1 != r {
              return false;
            }
            left = None;
            mask |= 1;
          } else {
            left = Some(c1);
            right = Some(c2);
          }
        }
        if (mask & 2) == 0 {
          counter[c1 as usize - 97] += 1;
          if counter[c1 as usize - 97] > 1 {
            mask |= 2;
          }
        }
      }
      mask != 0 && left.is_none()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    s: &'a str,
    goal: &'a str,
    ret: bool
  }

  #[test]
  fn test_buddy_strings_simple() {
    let suites = vec![
      Suite {
        s: "ab",
        goal: "ba",
        ret: true
      },
      Suite {
        s: "ab",
        goal: "ab",
        ret: false
      },
      Suite {
        s: "aa",
        goal: "aa",
        ret: true
      },
      Suite {
        s: "aaaaaaabc",
        goal: "aaaaaaacb",
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::buddy_strings(s.s.to_owned(), s.goal.to_owned()));
    }
  }
}