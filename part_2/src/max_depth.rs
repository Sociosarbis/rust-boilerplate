use super::*;

impl Solution {
  pub fn max_depth(s: String) -> i32 {
    let mut ret = 0;
    let mut temp = 0;
    for c in s.chars() {
      match c {
        '(' => {
          temp += 1;
          if temp > ret {
            ret = temp;
          }
        },
        ')' => {
          if temp > 0 {
            temp -= 1; 
          }
        },
        _ => {}
      }
    }
    ret
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
  fn test_max_depth_simple() {
    let suites = vec![
      Suite {
        s: "(1+(2*3)+((8)/4))+1",
        ret: 3
      },
      Suite {
        s: "(1)+((2))+(((3)))",
        ret: 3
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_depth(s.s.to_string()));
    }
  }
}