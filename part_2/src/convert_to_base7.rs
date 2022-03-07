use super::*;

impl Solution {
  pub fn convert_to_base7(mut num: i32) -> String {
    let sign = if num < 0 { 
      num = -num;
      -1 
    } else { 1 };
    let mut ret = 0;
    let mut base = 1;
    while num != 0 {
      ret += (num % 7) * base;
      num /= 7;
      base *= 10;
    }
    (ret * sign).to_string()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite<'a> {
    num: i32,
    ret: &'a str
  }

  #[test]
  fn test_convert_to_base7_simple() {
    let suites = vec![
      Suite {
        num: 100,
        ret: "202"
      },
      Suite {
        num: -7,
        ret: "-10"
      }
    ];

    for s in suites {
      assert_eq!(s.ret.to_string(), Solution::convert_to_base7(s.num));
    }
  }

}