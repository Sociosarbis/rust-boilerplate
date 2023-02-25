use super::*;

impl Solution {
  pub fn minimum_swap(s1: String, s2: String) -> i32 {
    let mut xy = 0;
    let mut yx = 0;
    let mut c1 = s1.chars();
    let mut c2 = s2.chars();
    let mut ret = 0;
    while let Some(a) = c1.next() {
      if let Some(b) = c2.next() {
        if a == 'x' && b == 'y' {
          xy += 1;
        } else if a == 'y' && b == 'x' {
          yx += 1;
        }
      }
    }
    ret += xy >> 1;
    xy &= 1;
    ret += yx >> 1;
    yx &= 1;
    if xy != yx {
      -1
    } else {
      ret + if xy == 0 { 0 } else { 2 }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    s1: String,
    s2: String,
    ret: i32
  }

  #[test]
  fn test_minimum_swap_simple() {
    let suites = vec![
      Suite {
        s1: "xx".to_string(),
        s2: "yy".to_string(),
        ret: 1
      },
      Suite {
        s1: "xy".to_string(),
        s2: "yx".to_string(),
        ret: 2
      },
      Suite {
        s1: "xx".to_string(),
        s2: "xy".to_string(),
        ret: -1
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::minimum_swap(s.s1, s.s2));
    }
  }
}