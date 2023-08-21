use super::*;

impl Solution {
  pub fn can_change(start: String, target: String) -> bool {
    let mut chars: Vec<char> = start.chars().collect();
    let mut i = 0;
    'a: for c in target.chars() {
      match c {
        'L' => {
          let old_i = i;
          while i < chars.len() {
            match chars[i] {
              'L' => {
                chars.swap(i, old_i);
                i = old_i + 1;
                continue 'a;
              }
              'R' => {
                return false;
              }
              _ => {}
            }
            i += 1;
          }
          return false;
        }
        _ => {}
      }
      i += 1;
    }
    i = chars.len() - 1;
    'a: for c in target.chars().rev() {
      match c {
        'R' => {
          let old_i = i;
          loop {
            match chars[i] {
              'R' => {
                chars.swap(i, old_i);
                if old_i > 0 {
                  i = old_i - 1;
                }
                continue 'a;
              }
              'L' => {
                return false;
              }
              _ => {}
            }
            if i > 0 {
              i -= 1;
            } else {
              return false;
            }
          }
        }
        _ => {
          if chars[i] != c {
            return false;
          }
        }
      }
      if i > 0 {
        i -= 1;
      }
    }
    for (i, c) in target.chars().enumerate() {
      if chars[i] != c {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    start: String,
    target: String,
    ret: bool,
  }

  #[test]
  fn test_can_change_simple() {
    let suites = vec![
      Suite {
        start: "_L__R__R_".to_string(),
        target: "L______RR".to_string(),
        ret: true,
      },
      Suite {
        start: "R_L_".to_string(),
        target: "__LR".to_string(),
        ret: false,
      },
      Suite {
        start: "_R".to_string(),
        target: "R_".to_string(),
        ret: false,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::can_change(s.start, s.target));
    }
  }
}
