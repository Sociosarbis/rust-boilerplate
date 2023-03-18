use super::*;

impl Solution {
  pub fn check_palindrome_formation(a: String, b: String) -> bool {
    if a.len() <= 1 || b.len() <= 1 {
      return true
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (mut l, mut r) = {
      if a.len() & 1 == 0 {
        (a.len() / 2 - 1, a.len() / 2)
      } else {
        (a.len() / 2 - 1, a.len() / 2 + 1)
      }
    };
    loop {
      if a[l] == a[r] {
        if l == 0 {
          return true;
        }
        l -= 1;
        r += 1;
      } else {
        let start_l = l;
        let start_r = r;
        loop {
          if a[l] == b[r] {
            if l == 0 {
              return true;
            }
            l -= 1;
            r += 1;
          } else {
            break;
          }
        }
        l = start_l;
        r = start_r;
        loop {
          if b[l] == a[r] {
            if l == 0 {
              return true;
            }
            l -= 1;
            r += 1;
          } else {
            break;
          }
        }
        break;
      }
    }
    let (mut l, mut r) = {
      if b.len() & 1 == 0 {
        (b.len() / 2 - 1, b.len() / 2)
      } else {
        (b.len() / 2 - 1, b.len() / 2 + 1)
      }
    };
    loop {
      if b[l] == b[r] {
        if l == 0 {
          return true;
        }
        l -= 1;
        r += 1;
      } else {
        let start_l = l;
        let start_r = r;
        loop {
          if a[l] == b[r] {
            if l == 0 {
              return true;
            }
            l -= 1;
            r += 1;
          } else {
            break;
          }
        }
        l = start_l;
        r = start_r;
        loop {
          if b[l] == a[r] {
            if l == 0 {
              return true;
            }
            l -= 1;
            r += 1;
          } else {
            break;
          }
        }
        break;
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    a: String,
    b: String,
    ret: bool
  }

  #[test]
  fn test_check_palindrome_formation_simple() {
    let suites = vec![
      Suite {
        a: "x".to_string(),
        b: "y".to_string(),
        ret: true
      },
      Suite {
        a: "xbdef".to_string(),
        b: "xecab".to_string(),
        ret: false
      },
      Suite {
        a: "ulacfd".to_string(),
        b: "jizalu".to_string(),
        ret: true
      },
      Suite {
        a: "pvhmupgqeltozftlmfjjde".to_string(),
        b: "yjgpzbezspnnpszebzmhvp".to_string(),
        ret: true
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::check_palindrome_formation(s.a, s.b));
    }
  }
}