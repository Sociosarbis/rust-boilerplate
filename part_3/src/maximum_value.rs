use super::*;

impl Solution {
  pub fn maximum_value(strs: Vec<String>) -> i32 {
    let mut ret = 0;
    for str in strs {
      if let Ok(v) = str.parse::<i32>() {
        if v > ret {
          ret = v;
        }
      } else {
        if str.len() as i32 > ret {
          ret = str.len() as i32;
        }
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    strs: Vec<String>,
    ret: i32,
  }

  #[test]
  fn test_maximum_value_simple() {
    let suites = vec![
      Suite {
        strs: t1!["alic3", "bob", "3", "4", "00000"],
        ret: 5,
      },
      Suite {
        strs: t1!["1", "01", "001", "0001"],
        ret: 1,
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::maximum_value(s.strs));
    }
  }
}
